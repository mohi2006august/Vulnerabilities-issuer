use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex, OnceLock},
};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use clap::{Parser, Subcommand, ValueEnum};
use regex::Regex;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

const MAX_BODY_BYTES: usize = 64 * 1024;
const MAX_FIELD_CHARS: usize = 2_000;

#[derive(Clone)]
struct AppState(Arc<Mutex<Connection>>);

#[derive(Deserialize)]
struct NewFinding {
    title: String,
    target: String,
    scanner: String,
    component: Option<String>,
    severity: Option<String>,
}

#[derive(Deserialize)]
struct ReviewDecision {
    decision: String,
}

#[derive(Parser)]
#[command(
    name = "bugraiser",
    about = "Find, review, and responsibly report vulnerabilities"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve,
    Findings {
        #[command(subcommand)]
        command: FindingCommand,
    },
}

#[derive(Subcommand)]
enum FindingCommand {
    Add {
        #[arg(long)]
        title: String,
        #[arg(long)]
        target: String,
        #[arg(long)]
        scanner: String,
        #[arg(long)]
        component: Option<String>,
        #[arg(long)]
        severity: Option<String>,
    },
    List,
    Review {
        id: String,
        #[arg(value_enum)]
        decision: Decision,
    },
}

#[derive(Clone, ValueEnum)]
enum Decision {
    Approved,
    Rejected,
    Duplicate,
    FalsePositive,
}

impl Decision {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Duplicate => "duplicate",
            Self::FalsePositive => "false-positive",
        }
    }
}

#[derive(Serialize)]
struct Finding {
    id: String,
    title: String,
    target: String,
    scanner: String,
    component: Option<String>,
    severity: Option<String>,
    status: String,
    created_at: i64,
}

struct ApiError(StatusCode, &'static str);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

impl AppState {
    fn open(path: &str) -> rusqlite::Result<Self> {
        let connection = Connection::open(path)?;
        connection.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS findings (
                 id TEXT PRIMARY KEY,
                 title TEXT NOT NULL,
                 target TEXT NOT NULL,
                 scanner TEXT NOT NULL,
                 component TEXT,
                 severity TEXT,
                 status TEXT NOT NULL,
                 created_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS audit_log (
                 id INTEGER PRIMARY KEY,
                 finding_id TEXT NOT NULL,
                 action TEXT NOT NULL,
                 created_at INTEGER NOT NULL,
                 FOREIGN KEY(finding_id) REFERENCES findings(id)
             );",
        )?;
        Ok(Self(Arc::new(Mutex::new(connection))))
    }

    fn connection(&self) -> Result<std::sync::MutexGuard<'_, Connection>, ApiError> {
        self.0
            .lock()
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))
    }

    fn add_finding(&self, input: NewFinding) -> Result<Finding, ApiError> {
        if !valid_text(&input.title)
            || !valid_text(&input.target)
            || !valid_text(&input.scanner)
            || input.component.as_deref().is_some_and(|v| !valid_text(v))
            || input.severity.as_deref().is_some_and(|v| !valid_text(v))
        {
            return Err(ApiError(
                StatusCode::BAD_REQUEST,
                "fields must be non-empty and at most 2000 characters",
            ));
        }

        let finding = Finding {
            id: Uuid::new_v4().to_string(),
            title: redact(&input.title),
            target: redact(&input.target),
            scanner: redact(&input.scanner),
            component: input.component.as_deref().map(redact),
            severity: input.severity.as_deref().map(redact),
            status: "needs-review".to_owned(),
            created_at: now_unix(),
        };
        let mut connection = self.connection()?;
        let transaction = connection
            .transaction()
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        transaction.execute(
            "INSERT INTO findings (id, title, target, scanner, component, severity, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![finding.id, finding.title, finding.target, finding.scanner, finding.component, finding.severity, finding.status, finding.created_at],
        ).map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        transaction
            .execute(
                "INSERT INTO audit_log (finding_id, action, created_at) VALUES (?1, 'created', ?2)",
                params![finding.id, finding.created_at],
            )
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        transaction
            .commit()
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        Ok(finding)
    }

    fn list_findings(&self) -> Result<Vec<Finding>, ApiError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare("SELECT id, title, target, scanner, component, severity, status, created_at FROM findings ORDER BY created_at DESC")
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        statement
            .query_map([], read_finding)
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))
    }

    fn decide_finding(&self, id: &str, decision: &str) -> Result<Finding, ApiError> {
        if !matches!(
            decision,
            "approved" | "rejected" | "duplicate" | "false-positive"
        ) {
            return Err(ApiError(
                StatusCode::BAD_REQUEST,
                "decision must be approved, rejected, duplicate, or false-positive",
            ));
        }

        let mut connection = self.connection()?;
        let transaction = connection
            .transaction()
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        let changed = transaction
            .execute(
                "UPDATE findings SET status = ?1 WHERE id = ?2 AND status = 'needs-review'",
                params![decision, id],
            )
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        if changed == 0 {
            let exists = transaction
                .query_row("SELECT 1 FROM findings WHERE id = ?1", [id], |_| Ok(()))
                .optional()
                .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?
                .is_some();
            return Err(if exists {
                ApiError(StatusCode::CONFLICT, "finding already reviewed")
            } else {
                ApiError(StatusCode::NOT_FOUND, "finding not found")
            });
        }
        let timestamp = now_unix();
        transaction
            .execute(
                "INSERT INTO audit_log (finding_id, action, created_at) VALUES (?1, ?2, ?3)",
                params![id, decision, timestamp],
            )
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        let finding = transaction
            .query_row("SELECT id, title, target, scanner, component, severity, status, created_at FROM findings WHERE id = ?1", [id], read_finding)
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        transaction
            .commit()
            .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
        Ok(finding)
    }
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn valid_text(value: &str) -> bool {
    !value.trim().is_empty() && value.chars().count() <= MAX_FIELD_CHARS
}

fn redact(value: &str) -> String {
    static PATTERNS: OnceLock<[Regex; 3]> = OnceLock::new();
    let patterns = PATTERNS.get_or_init(|| {
        [
            Regex::new(r"(?i)(bearer\s+)[A-Za-z0-9._~+/-]+=*").unwrap(),
            Regex::new(r#"(?i)(api[_-]?key|access[_-]?token|secret|password)\s*[:=]\s*[\"']?[^,\s\"';&]+\"?"#).unwrap(),
            Regex::new(r"\b(?:gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|AKIA[0-9A-Z]{16})\b").unwrap(),
        ]
    });
    let redacted = patterns[0].replace_all(value, "$1[REDACTED]");
    let redacted = patterns[1].replace_all(&redacted, "$1=[REDACTED]");
    patterns[2]
        .replace_all(&redacted, "[REDACTED]")
        .into_owned()
}

fn read_finding(row: &rusqlite::Row<'_>) -> rusqlite::Result<Finding> {
    Ok(Finding {
        id: row.get(0)?,
        title: row.get(1)?,
        target: row.get(2)?,
        scanner: row.get(3)?,
        component: row.get(4)?,
        severity: row.get(5)?,
        status: row.get(6)?,
        created_at: row.get(7)?,
    })
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn list_findings(State(state): State<AppState>) -> Result<Json<Vec<Finding>>, ApiError> {
    Ok(Json(state.list_findings()?))
}

async fn create_finding(
    State(state): State<AppState>,
    Json(input): Json<NewFinding>,
) -> Result<(StatusCode, Json<Finding>), ApiError> {
    let finding = state.add_finding(input)?;
    Ok((StatusCode::CREATED, Json(finding)))
}

async fn decide_finding(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<ReviewDecision>,
) -> Result<Json<Finding>, ApiError> {
    Ok(Json(state.decide_finding(&id, &input.decision)?))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let database_path =
        env::var("BUGRAISER_DATABASE").unwrap_or_else(|_| "bugraiser.db".to_owned());
    let state = AppState::open(&database_path)?;
    match cli.command {
        Command::Serve => serve(state).await?,
        Command::Findings { command } => match command {
            FindingCommand::Add {
                title,
                target,
                scanner,
                component,
                severity,
            } => {
                let finding = state
                    .add_finding(NewFinding {
                        title,
                        target,
                        scanner,
                        component,
                        severity,
                    })
                    .map_err(|error| std::io::Error::other(error.1))?;
                println!(
                    "Created {} ({}) — {}",
                    finding.id, finding.status, finding.title
                );
            }
            FindingCommand::List => {
                let findings = state
                    .list_findings()
                    .map_err(|error| std::io::Error::other(error.1))?;
                if findings.is_empty() {
                    println!("No findings yet.");
                } else {
                    println!("ID | STATUS | SEVERITY | TITLE | TARGET");
                    for finding in findings {
                        println!(
                            "{} | {} | {} | {} | {}",
                            finding.id,
                            finding.status,
                            finding.severity.as_deref().unwrap_or("-"),
                            finding.title,
                            finding.target
                        );
                    }
                }
            }
            FindingCommand::Review { id, decision } => {
                let finding = state
                    .decide_finding(&id, decision.as_str())
                    .map_err(|error| std::io::Error::other(error.1))?;
                println!("{} marked {}.", finding.id, finding.status);
            }
        },
    }
    Ok(())
}

async fn serve(state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("BUGRAISER_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);
    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/findings", get(list_findings).post(create_finding))
        .route("/api/v1/findings/{id}/decision", post(decide_finding))
        .layer(axum::extract::DefaultBodyLimit::max(MAX_BODY_BYTES))
        .with_state(state);
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(address).await?;
    println!("BugRaiser API listening on http://{address}");
    axum::serve(listener, app).await?;
    Ok(())
}
