# Roadmap

Dates are targets, not promises. Start from the safest, highest-value pieces.

## Milestone 0: Foundations (weeks 1–2)
- [x] Language: Rust. License: MIT OR Apache-2.0
- [ ] Choose the final project name and register it on crates.io
- [ ] Cargo workspace, `rust-toolchain.toml`, `deny.toml`, CI (fmt, clippy, test, cargo-deny, cargo-audit), pre-commit
- [x] Issue and PR templates, CODEOWNERS, governance docs (drafted)
- [ ] Publish `README`, `RESPONSIBLE_USE`, `DISCLOSURE_POLICY`, `SECURITY`, `CODE_OF_CONDUCT`
- [ ] Set up a private security contact and mailing list

## Milestone 1: MVP, open-source repos (weeks 3–8)
- [ ] CLI skeleton and config loader
- [ ] Findings store (SQLite) and JSON/SARIF schema
- [ ] `deps` scanner (OSV-Scanner / Trivy wrapper)
- [ ] `sast` scanner (Semgrep wrapper, SARIF import)
- [ ] Deduplicator against OSV and GitHub advisories and issues
- [ ] Report builder with templates A and B
- [ ] Review gate (terminal UI)
- [ ] GitHub adapter: public issues + Private Vulnerability Reporting handoff
- [ ] Audit log
- [ ] Test fixtures (intentionally vulnerable sample apps)

## Milestone 2: Websites (weeks 9–14)
- [ ] Authorization file format and gate
- [ ] Rate limiter and robots.txt handling
- [ ] `web-passive` scanner: headers, TLS, cookies, CORS, exposed files
- [ ] `security.txt` and VDP / bug bounty discovery
- [ ] Email adapter and website report template (C)

## Milestone 3: Installed software (weeks 15–18)
- [ ] `scan host --local`: package manager inventory (dpkg, rpm, brew, winget, pip, npm)
- [ ] SBOM import/export (CycloneDX, SPDX)
- [ ] Vendor contact discovery and routing

## Milestone 4: Ecosystem (weeks 19–26)
- [ ] Plugin SDK and registry
- [ ] GitLab, Codeberg, Bugzilla, Jira adapters
- [ ] HackerOne / Bugcrowd integration (draft only)
- [ ] Release automation (cargo-dist): signed binaries, checksums, crates.io publish
- [ ] GitHub Action and Docker image
- [ ] Fuzz targets (cargo-fuzz) for every parser of untrusted input
- [ ] Optional `web-active` scanner with strict opt-in
- [ ] Follow-up tracker and deadline reminders
- [ ] Public advisory generator

## Milestone 5: Hardening and 1.0
- [ ] Third-party security review of the safety layer
- [ ] Documentation site (mdBook) and published rustdoc for the SDK
- [ ] Localization of templates
- [ ] Stable plugin API (semver)
- [ ] 1.0 release

## Ideas backlog
- Fuzzing harness integration (OSS-Fuzz style)
- AI-assisted triage and report drafting (always human-reviewed)
- Maintainer-side mode: receive and triage reports
- Reputation and rate controls per maintainer to prevent spam
- Web dashboard
