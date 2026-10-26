# Configuration Reference

Config is TOML. BugRaiser reads, in increasing priority:

1. `~/.config/bugraiser/config.toml` (user)
2. `./bugraiser.toml` (project)
3. Environment variables prefixed `BUGRAISER_`, with `__` separating nested keys, e.g. `BUGRAISER_SAFETY__RATE_LIMIT_RPS=2`
4. Command-line flags

Never put tokens in config files. Use environment variables or your OS keychain.

```toml
version = 1

[identity]
name = "Your Name"
handle = "yourhandle"
contact_email = "you@example.com"
credit_preference = "named"        # named | handle | anonymous

[safety]
require_authorization = true       # release builds refuse to start if false
rate_limit_rps = 5
max_concurrent_hosts = 2
respect_robots_txt = true
user_agent = "BugRaiser/{version} (+https://github.com/<org>/bugraiser)"
redact_secrets = true              # release builds refuse to start if false
max_response_bytes = 10_000_000
request_timeout_seconds = 30

[scanners]
enabled = ["deps", "sast", "secrets", "web-passive"]

[scanners.web_active]
enabled = false                    # opt-in, authorized targets only

[scanners.deps]
backends = ["osv-scanner", "trivy"]

[scanners.sast]
backends = ["semgrep"]

[verification]
min_confidence = 0.7
require_manual_repro = true

[dedup]
sources = ["osv", "nvd", "ghsa", "repo-issues", "repo-advisories"]
similarity_threshold = 0.82

[disclosure]
deadline_days = 90
grace_days = 14
followup_after_days = 7

[review]
require_human_approval = true      # release builds refuse to start if false
editor = "$EDITOR"

[adapters.github]
token_env = "GITHUB_TOKEN"         # fine-grained token with minimal scopes

[adapters.gitlab]
token_env = "GITLAB_TOKEN"

[adapters.smtp]
host = "smtp.example.com"
port = 587
username_env = "SMTP_USER"
password_env = "SMTP_PASS"

[storage]
path = "~/.local/share/bugraiser/findings.db"
evidence_dir = "~/.local/share/bugraiser/evidence"
audit_log = "~/.local/share/bugraiser/audit.jsonl"
```

Safety settings marked above are developer-overridable only in debug builds, so tests can run. This is a guard against accidental misconfiguration, not a security boundary: anyone can compile their own build, which is why the license and Responsible Use Policy matter.

## CLI commands

| Command | Purpose |
|---|---|
| `bugraiser init` | Create config and walk through the responsible-use acknowledgment |
| `bugraiser scan repo <url\|path>` | Scan source and dependencies |
| `bugraiser scan site <url> --authorized-by <file>` | Authorized web scan |
| `bugraiser scan host --local` | Audit software installed on this machine |
| `bugraiser findings list` | List stored findings |
| `bugraiser review` | Interactive triage |
| `bugraiser raise --finding <id> [--dry-run] [--channel <name> --to <addr>]` | Build, route, and (after approval) submit |
| `bugraiser status` | Track acknowledgments and deadlines |
| `bugraiser disclose --finding <id>` | Produce the public advisory |
| `bugraiser data purge ...` | Delete stored findings, evidence, or everything |
| `bugraiser stop` | Abort running scans |

Global flags: `--config <path>`, `--offline`, `--keep-work`, `--verbose`, `--quiet`. Logging is controlled with `RUST_LOG` (for example `RUST_LOG=bugraiser=debug`).

## Tokens and permissions

- GitHub: use a fine-grained token with only the scopes you need. External reporters normally use Private Vulnerability Reporting through the web flow, which the tool opens for you; API write access to someone else's repository is usually not available and not needed.
- Never commit tokens. The repo ships a `gitleaks` pre-commit hook.
