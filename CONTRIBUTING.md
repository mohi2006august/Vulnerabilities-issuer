# Contributing to BugRaiser

Thanks for helping! This project handles sensitive material, so a few extra rules apply.

## Ways to contribute

- New scanner plugins
- Tracker and platform adapters (GitLab, Codeberg, Bugzilla, Jira, etc.)
- Report templates and translations
- Documentation
- Bug reports and test cases
- Reviewing the safety mechanisms (most valuable)

## Getting started

You need a recent stable Rust toolchain (the exact version is pinned in `rust-toolchain.toml`).

```bash
git clone https://github.com/<org>/bugraiser
cd bugraiser
cargo build --workspace
cargo test --workspace
```

Recommended tools:

```bash
cargo install cargo-deny cargo-audit cargo-nextest   # dependency policy, advisories, faster tests
cargo install cargo-fuzz                             # for parsers that read untrusted input
pre-commit install                                   # runs fmt, clippy, and gitleaks before commits
```

## Workflow

1. Open or claim an issue before starting large work.
2. Branch from `main`: `feat/<name>` or `fix/<name>`.
3. Keep PRs focused and under about 400 lines when possible.
4. Add tests and docs, and an entry in `CHANGELOG.md` under *Unreleased*.
5. Run these locally:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   cargo deny check
   ```
6. Open a PR using the template and link the issue.

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`).

## Developer Certificate of Origin

Sign off every commit (`git commit -s`). By doing so you certify the [DCO](https://developercertificate.org/).

## Licensing of contributions

BugRaiser is licensed under **MIT OR Apache-2.0**. Unless you state otherwise, any contribution you submit is dual-licensed under the same terms, without additional conditions.

## Rules specific to this project

1. **Never test against real third-party targets in tests or CI.** Use the bundled intentionally-vulnerable fixtures (`tests/fixtures/vuln-app/`) or your own infrastructure.
2. **Do not weaken safety checks.** PRs that remove or bypass the authorization gate, rate limiter, redaction, or human-review step will be rejected.
3. **No weaponized exploits.** Scanners detect and confirm; they do not ship payloads for gaining access, persistence, or data extraction.
4. **No real secrets or personal data** in fixtures, logs, or issues.
5. **Plugins must declare** whether they need network access and authorization, and must use the SDK's HTTP client rather than making their own requests.

## Keep it minimal

Read [AGENTS.md](AGENTS.md). It sets the rules for keeping changes small and avoiding unnecessary code, and it applies to AI-assisted contributions too.

## Rust guidelines

- Core crates use `#![forbid(unsafe_code)]`. If `unsafe` is truly needed, isolate it in a small module with a safety comment and get review from a safety owner.
- No `unwrap()` or `expect()` on anything derived from a target (files, HTTP responses, repository contents). Return errors.
- Libraries use `thiserror`; the CLI binary uses `anyhow`.
- Use `tracing` for logs. Never log secrets, tokens, or unredacted evidence.
- All network access goes through the shared client in `bugraiser-core` (scope check, rate limit, audit).
- Parsers of untrusted input need size limits and, where practical, a fuzz target.
- Public items have doc comments (`#![warn(missing_docs)]` in library crates).
- Prefer small pure functions; keep I/O at the edges.
- MSRV changes need a note in `CHANGELOG.md`.
- Add dependencies sparingly; each one expands our supply-chain risk. New dependencies must pass `cargo deny` and be justified in the PR.

## Writing a scanner plugin

Follow `docs/PLUGIN_GUIDE.md`. In short:

1. Implement the `Scanner` trait from `bugraiser-sdk`, or write an external plugin with a manifest.
2. Emit findings in the standard schema, with a confidence value and, where possible, a deterministic reproduction.
3. Add unit tests with fixtures and a README section describing what it does and does not detect.
4. Document its network behavior and request rate.
5. Update `docs/THREAT_MODEL.md` if it introduces new risks.

## Review process

At least one maintainer approves. Changes to safety-critical areas (see `GOVERNANCE.md`) need two approvals including a safety owner.

## Reporting problems

- Bugs: open a GitHub issue.
- Security issues in BugRaiser: see [SECURITY.md](SECURITY.md). Do **not** open a public issue.
- Conduct concerns: see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
