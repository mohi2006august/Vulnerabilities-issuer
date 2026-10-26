# Architecture

## 1. Goals

- **Modular**: scanners, trackers, and report formats are plugins.
- **Safe by default**: authorization checks, rate limits, human approval.
- **Rust core, language-neutral plugins**: findings exchanged as JSON (SARIF-compatible), so scanners can be written in any language.
- **Auditable**: every action is logged.

## 2. Pipeline

```
 Target ──► Authorization Gate ──► Scanner Plugins ──► Findings Store
                                                          │
                                  Verifier ◄──────────────┘
                                     │
                                  Deduplicator ──► Severity Scorer (CVSS 4.0)
                                     │
                                  Report Builder ──► Human Review Gate
                                                          │
                                  Disclosure Router ◄─────┘
                                     │
                          Tracker / Email / Advisory Adapters
                                     │
                                 Audit Log
```

## 3. Components

| Component | Responsibility |
|---|---|
| **CLI / API** | `bugraiser` commands, optional local REST API, later a web UI |
| **Authorization Gate** | Validates `authorization.toml`, enforces scope and expiry |
| **Scanner plugins** | Produce raw findings. One plugin per technique |
| **Findings Store** | SQLite by default, Postgres optional. Stores findings, evidence, status |
| **Verifier** | Re-runs checks, applies confidence scoring, filters false positives |
| **Deduplicator** | Queries OSV, NVD, GHSA, repo issues and advisories; fuzzy matches titles and affected components |
| **Severity Scorer** | CVSS 4.0 calculator plus EPSS and KEV context |
| **Report Builder** | Renders Markdown from templates |
| **Review Gate** | Interactive approve / edit / reject. Required before submission |
| **Disclosure Router** | Implements the routing order in `DISCLOSURE_POLICY.md` |
| **Adapters** | GitHub, GitLab, Codeberg, email (SMTP), HackerOne, Bugcrowd, security.txt |
| **Audit Log** | Append-only JSONL: who, what, when, target, authorization reference |

## 4. Scanner categories

| Category | Target | Example backends |
|---|---|---|
| `deps` | Repos, lockfiles, SBOMs, installed packages | OSV-Scanner, Trivy, Grype, pip-audit, npm audit |
| `sast` | Source code | Semgrep, CodeQL (SARIF import), Bandit, gosec |
| `secrets` | Repos | gitleaks, trufflehog (report only exposure, never the value) |
| `web-passive` | Websites | Header analysis, TLS (testssl), cookie flags, CORS, `security.txt`, exposed files |
| `web-active` *(opt-in, authorized only)* | Websites | ZAP baseline, Nuclei with non-intrusive templates |
| `bugs` | Repos and software | Linters, test failures, crash reproducers, fuzzing results |

## 5. Plugin API (draft)

Scanners are either **Rust crates** implementing a trait from `bugraiser-sdk`, or **external executables** that speak a JSON protocol over stdin/stdout. Full details and a worked example are in `PLUGIN_GUIDE.md`.

```rust
#[async_trait]
pub trait Scanner: Send + Sync {
    fn name(&self) -> &'static str;
    fn category(&self) -> Category;          // Deps | Sast | Secrets | WebPassive | WebActive | Bugs
    fn requires_authorization(&self) -> bool;

    async fn scan(&self, target: &Target, ctx: &ScanContext) -> Result<Vec<Finding>, ScanError>;
}
```

Plugins never open their own network connections. `ScanContext::http()` returns a client that enforces scope, rate limits, size limits, and audit logging.

Finding schema:

```json
{
  "id": "F-001",
  "scanner": "osv-deps",
  "title": "Prototype pollution in lodash < 4.17.21",
  "kind": "vulnerability",
  "target": "github.com/org/project",
  "component": "lodash@4.17.15",
  "location": {"file": "package-lock.json", "line": 412},
  "severity": {"cvss_v4": "7.5", "vector": "..."},
  "confidence": 0.9,
  "references": ["GHSA-xxxx", "CVE-2021-23337"],
  "evidence": ["evidence/F-001-trace.txt"],
  "status": "needs-review"
}
```

Statuses: `new → verified → needs-review → approved → submitted → acknowledged → fixed → disclosed` (or `rejected`, `duplicate`, `false-positive`).

## 6. Safety mechanisms

- Scanners that touch the network cannot start without a valid authorization.
- Global and per-host rate limiter.
- Secrets redacted before anything is written to disk.
- A kill switch: `bugraiser stop` aborts all running scans.
- Dry-run mode (`--dry-run`) renders reports and routing decisions without sending anything.

## 7. Suggested tech stack

- **Language**: Rust (stable, MSRV pinned), `#![forbid(unsafe_code)]` in core crates
- **CLI and TUI**: `clap`, `ratatui`
- **Async and HTTP**: `tokio`, `reqwest` with `rustls` (no OpenSSL dependency)
- **Rate limiting**: `governor`
- **Serialization**: `serde`, `serde_json`, `toml`
- **Storage**: SQLite via `rusqlite` or `sqlx`, Postgres optional later
- **Templates**: `minijinja` or `tera`
- **Errors and logging**: `thiserror`, `anyhow`, `tracing`
- **Plugins**: Rust crates via `bugraiser-sdk`, or subprocess + JSON so any language works
- **Supply chain**: `cargo-deny`, `cargo-audit`, committed `Cargo.lock`, pinned CI actions
- **Testing**: `cargo-nextest`, `proptest` for parsers, `cargo-fuzz` for untrusted input
- **Release**: `cargo-dist` (prebuilt binaries, checksums), crates.io, Homebrew, Docker image
- **Docs**: `mdBook`, plus rustdoc for the SDK

## 8. Suggested repository layout

```
bugraiser/
├── Cargo.toml                    # workspace
├── Cargo.lock                    # committed
├── rust-toolchain.toml
├── deny.toml                     # cargo-deny policy
├── crates/
│   ├── bugraiser-cli/            # binary: clap commands, TUI review
│   ├── bugraiser-core/           # pipeline, store, audit
│   │   └── src/safety/           # authorization gate, scope, rate limiter (safety-critical)
│   │   └── src/redact/           # secret redaction (safety-critical)
│   ├── bugraiser-scanners/       # built-in scanners
│   ├── bugraiser-routers/        # disclosure routing
│   ├── bugraiser-adapters/       # GitHub, GitLab, SMTP, ...
│   └── bugraiser-sdk/            # public plugin API and types
├── templates/                    # report templates
├── tests/
│   └── fixtures/vuln-app/        # intentionally vulnerable local test target
├── fuzz/                         # cargo-fuzz targets
├── docs/
├── examples/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   ├── workflows/
│   ├── CODEOWNERS
│   └── PULL_REQUEST_TEMPLATE.md
├── README.md  CONTRIBUTING.md  SECURITY.md  CODE_OF_CONDUCT.md
├── GOVERNANCE.md  MAINTAINERS.md  ROADMAP.md  CHANGELOG.md
└── LICENSE-MIT  LICENSE-APACHE
```
