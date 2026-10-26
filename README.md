# BugRaiser

An open-source tool that helps you **find, verify, and responsibly report** vulnerabilities and bugs in:

- **Open-source projects** (GitHub, GitLab, Codeberg, etc.): scan code and dependencies, then open well-formed issues or private security advisories.
- **Installed software** (packages, binaries, containers): match versions against known vulnerability databases and file reports with vendors.
- **Websites and web apps**: non-intrusive checks, run **only against targets you own or are explicitly authorized to test**.

> BugRaiser does not exploit anything. It detects, verifies, deduplicates, and drafts reports. A human reviews every report before it is sent.

---

## Why this exists

Finding a bug is easy. Reporting it well is hard: you need to check for duplicates, find the right contact, avoid leaking the issue publicly, and write a clear, reproducible report. BugRaiser automates the tedious parts and keeps a human in control.

## Core features (planned)

| Area | What it does |
|---|---|
| Scanners | Pluggable modules: dependency audit (OSV, NVD, GHSA), static analysis (Semgrep, CodeQL SARIF import), web header/TLS/config checks |
| Verification | Confidence scoring, reproduction steps, false-positive filtering |
| Deduplication | Searches existing issues, advisories, and CVEs before anything is drafted |
| Report builder | Generates reports from templates (severity via CVSS 4.0, impact, PoC, suggested fix) |
| Disclosure router | Finds `SECURITY.md`, `security.txt`, GitHub Private Vulnerability Reporting, or vendor program |
| Human review gate | Nothing is submitted without explicit approval |
| Audit log | Records what was scanned, when, and under which authorization |

## Quick start (target UX)

```bash
cargo install bugraiser --locked   # or download a prebuilt binary from Releases

bugraiser init                   # creates config + consent file
bugraiser scan repo https://github.com/org/project
bugraiser scan site https://example.com --authorized-by ./authorization.toml
bugraiser review                 # interactive triage of findings
bugraiser raise --finding F-001  # drafts and, after approval, submits
```

## Documentation

| Doc | Purpose |
|---|---|
| [docs/RESPONSIBLE_USE.md](docs/RESPONSIBLE_USE.md) | **Read first.** Rules, scope, authorization |
| [docs/DISCLOSURE_POLICY.md](docs/DISCLOSURE_POLICY.md) | How reports are timed and routed |
| [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md) | Install and first scan |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design (Rust workspace) |
| [docs/PLUGIN_GUIDE.md](docs/PLUGIN_GUIDE.md) | Writing scanner plugins |
| [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md) | Risks and mitigations |
| [docs/PRIVACY.md](docs/PRIVACY.md) | What data is stored and sent |
| [docs/REPORT_TEMPLATE.md](docs/REPORT_TEMPLATE.md) | Vulnerability and bug report templates |
| [docs/CONFIGURATION.md](docs/CONFIGURATION.md) | Config file and CLI reference |
| [docs/FAQ.md](docs/FAQ.md) | Common questions |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |
| [AGENTS.md](AGENTS.md) | Minimal-code rules for contributors and AI assistants |
| [GOVERNANCE.md](GOVERNANCE.md) | Roles and decision making |
| [CHANGELOG.md](CHANGELOG.md) | Release history |
| [SECURITY.md](SECURITY.md) | Reporting vulnerabilities in BugRaiser itself |
| [ROADMAP.md](ROADMAP.md) | Milestones |
| [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | Community standards |

## Backend API (early development)

The initial Rust backend provides a loopback-only API and SQLite store for findings. Start it with `cargo run`; see [docs/API.md](docs/API.md) for routes and configuration. Findings require an explicit review decision, and the API does not submit reports.

## Ground rules (summary)

1. Only test what you own or have **written permission** to test.
2. No exploitation beyond the minimum proof needed. No data access, no denial of service.
3. Report privately first. Follow coordinated disclosure.
4. Never spam maintainers: one verified, deduplicated, well-written report beats fifty automated ones.

## License

Dual-licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option. Unless you state otherwise, contributions you submit are dual-licensed the same way, without additional terms.

## Status

Pre-alpha, written in Rust. The local findings API is an early backend slice; scanners, authentication, and report submission are not implemented. See [ROADMAP.md](ROADMAP.md).
