# Plugin Guide

BugRaiser supports two kinds of scanner plugins:

| Kind | Language | Best for |
|---|---|---|
| **Built-in / crate plugin** | Rust, using `bugraiser-sdk` | Fast, deep integration, official scanners |
| **External plugin** | Anything | Wrapping existing tools, quick experiments |

> The SDK API below is a **draft** and will change before 1.0.

## Rules every plugin must follow

1. **Never make network requests directly.** Use the HTTP client from the scan context. It enforces the authorization scope, the rate limiter, the user agent, and audit logging. External plugins that need the network must declare it and are given a proxy.
2. **Detect, do not exploit.** No payloads that gain access, change data, or cause outages.
3. **Declare** whether you need network access and authorization.
4. **Report a confidence** between 0.0 and 1.0 and a deterministic reproduction where possible.
5. **Never write secrets** into findings. Report location and type, not the value.

## Rust plugin

Crate naming convention: `bugraiser-scanner-<name>`.

```toml
# Cargo.toml
[package]
name = "bugraiser-scanner-security-txt"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"

[dependencies]
bugraiser-sdk = "0.1"
async-trait = "0.1"
```

### The trait

```rust
#[async_trait]
pub trait Scanner: Send + Sync {
    fn name(&self) -> &'static str;
    fn category(&self) -> Category;           // Deps | Sast | Secrets | WebPassive | WebActive | Bugs
    fn requires_authorization(&self) -> bool;

    async fn scan(
        &self,
        target: &Target,
        ctx: &ScanContext,
    ) -> Result<Vec<Finding>, ScanError>;
}
```

### Worked example: check for `security.txt`

```rust
use async_trait::async_trait;
use bugraiser_sdk::{
    Category, Finding, FindingKind, ScanContext, ScanError, Scanner, Severity, Target,
};

pub struct SecurityTxtScanner;

#[async_trait]
impl Scanner for SecurityTxtScanner {
    fn name(&self) -> &'static str { "security-txt" }
    fn category(&self) -> Category { Category::WebPassive }
    fn requires_authorization(&self) -> bool { true }

    async fn scan(&self, target: &Target, ctx: &ScanContext) -> Result<Vec<Finding>, ScanError> {
        let url = target.url()?.join("/.well-known/security.txt")?;

        // Rate-limited, scope-checked, logged.
        let resp = ctx.http().get(url.clone()).send().await?;
        if resp.status().is_success() {
            return Ok(vec![]);
        }

        Ok(vec![Finding::builder()
            .scanner(self.name())
            .kind(FindingKind::Hardening)
            .title("No security.txt published")
            .location(url.as_str())
            .severity(Severity::Info)
            .confidence(0.95)
            .description("Researchers have no documented way to report vulnerabilities privately.")
            .recommendation("Publish /.well-known/security.txt (RFC 9116).")
            .build()?])
    }
}
```

### Registering

```rust
pub fn register(registry: &mut bugraiser_sdk::Registry) {
    registry.add(Box::new(SecurityTxtScanner));
}
```

### Testing

- Unit-test parsing and decision logic with fixtures.
- Integration-test against `tests/fixtures/vuln-app/` (a local intentionally vulnerable app), never real third-party sites.
- Use the SDK's `MockContext` to assert that your plugin only calls `ctx.http()` for in-scope URLs.

## External plugin (any language)

An external plugin is an executable plus a manifest. BugRaiser starts it as a subprocess, writes one JSON request to its stdin, and reads findings as newline-delimited JSON from stdout. Logs go to stderr.

### Manifest

```toml
# bugraiser-plugin.toml
name = "my-scanner"
version = "0.1.0"
category = "sast"
requires_authorization = false
network = false                 # true => you receive a proxy URL; direct access is blocked
command = ["./my-scanner"]
```

### Request (stdin)

```json
{
  "protocol": 1,
  "target": { "kind": "repo", "path": "/tmp/bugraiser/work/abc123" },
  "limits": { "timeout_seconds": 300, "max_findings": 500 },
  "proxy": null
}
```

### Findings (stdout, one JSON object per line)

```json
{"title":"Hardcoded password","kind":"vulnerability","component":"app/config.py","location":{"file":"app/config.py","line":14},"severity":{"label":"high"},"confidence":0.8,"description":"...","recommendation":"..."}
```

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Completed (with or without findings) |
| 2 | Invalid request |
| 3 | Missing dependency (message on stderr) |
| other | Failed; findings already emitted are kept |

### Sandboxing

External plugins run with a minimal environment (no inherited tokens), a working directory limited to the scan target, a timeout, and, where the OS supports it, filesystem and network restrictions (Landlock/seccomp on Linux, sandbox profiles on macOS). Sandboxing is best-effort. Only install plugins you trust.

## Publishing

1. Choose a clear name: `bugraiser-scanner-<name>`.
2. Document what it detects, what it does not, its requests per target, and its false-positive rate.
3. License it (MIT OR Apache-2.0 is recommended for compatibility).
4. Publish to crates.io, or list your external plugin in `docs/plugins.md` via PR. Listing does not mean endorsement, and maintainers may remove plugins that violate the Responsible Use Policy.
