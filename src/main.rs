use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let connection = state.connection()?;
    let mut statement = connection
        .prepare("SELECT id, title, target, scanner, component, severity, status, created_at FROM findings ORDER BY created_at DESC")
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
    let findings = statement
        .query_map([], read_finding)
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
    Ok(Json(findings))
}

async fn create_finding(
    State(state): State<AppState>,
    Json(input): Json<NewFinding>,
) -> Result<(StatusCode, Json<Finding>), ApiError> {
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
        title: input.title,
        target: input.target,
        scanner: input.scanner,
        component: input.component,
        severity: input.severity,
        status: "needs-review".to_owned(),
        created_at: now_unix(),
    };
    let mut connection = state.connection()?;
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
    Ok((StatusCode::CREATED, Json(finding)))
}

async fn decide_finding(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<ReviewDecision>,
) -> Result<Json<Finding>, ApiError> {
    if !matches!(
        input.decision.as_str(),
        "approved" | "rejected" | "duplicate" | "false-positive"
    ) {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "decision must be approved, rejected, duplicate, or false-positive",
        ));
    }

    let mut connection = state.connection()?;
    let transaction = connection
        .transaction()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
    let changed = transaction
        .execute(
            "UPDATE findings SET status = ?1 WHERE id = ?2 AND status = 'needs-review'",
            params![input.decision, id],
        )
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
    if changed == 0 {
        let exists = transaction
            .query_row("SELECT 1 FROM findings WHERE id = ?1", [&id], |_| Ok(()))
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
            params![id, input.decision, timestamp],
        )
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
    let finding = transaction
        .query_row("SELECT id, title, target, scanner, component, severity, status, created_at FROM findings WHERE id = ?1", [&id], read_finding)
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
    transaction
        .commit()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "storage unavailable"))?;
    Ok(Json(finding))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_path =
        env::var("BUGRAISER_DATABASE").unwrap_or_else(|_| "bugraiser.db".to_owned());
    let port = env::var("BUGRAISER_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);
    let state = AppState::open(&database_path)?;
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
