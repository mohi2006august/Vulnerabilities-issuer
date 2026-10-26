# Getting Started

> BugRaiser is pre-alpha. This guide describes the **target** experience for v0.1 and will be corrected as the code lands. Treat commands as the design, not a promise.

## 1. Install

Pick one:

```bash
# From crates.io (needs a recent stable Rust toolchain)
cargo install bugraiser --locked

# From source
git clone https://github.com/<org>/bugraiser
cd bugraiser
cargo install --path crates/bugraiser-cli --locked

# Prebuilt binary: download from the GitHub Releases page, verify the checksum
# and signature, then place it on your PATH.

# Docker
docker run --rm -v "$PWD:/work" ghcr.io/<org>/bugraiser --help
```

The minimum supported Rust version (MSRV) is recorded in the workspace `Cargo.toml` (`rust-version`).

Optional external tools improve results. BugRaiser detects them and tells you what is missing:

| Tool | Used for |
|---|---|
| `osv-scanner` or `trivy` | Dependency vulnerability scanning |
| `semgrep` | Static analysis rules |
| `gitleaks` | Secret exposure detection |
| `git` | Cloning repositories |

## 2. Initialize

```bash
bugraiser init
```

This creates `~/.config/bugraiser/config.toml`, asks for your name and contact email (used in reports), and asks you to read and accept `docs/RESPONSIBLE_USE.md`.

Set tokens as environment variables, never in files:

```bash
export GITHUB_TOKEN=...     # fine-grained, minimal scopes
```

## 3. Scan an open-source repository

```bash
bugraiser scan repo https://github.com/org/project
bugraiser findings list
```

BugRaiser clones into a temporary directory, runs the enabled scanners, and stores findings locally. Nothing is sent anywhere.

## 4. Scan a website (authorized targets only)

Create an authorization file (see `docs/RESPONSIBLE_USE.md`):

```toml
# authorization.toml
target = "example.com"
authorized_by = "Jane Doe, CTO"
contact = "security@example.com"
valid_from = 2026-10-26
valid_until = 2026-11-26
evidence = "https://example.com/security-policy"

[scope]
include = ["example.com", "*.example.com"]
exclude = ["payments.example.com"]
```

```bash
bugraiser scan site https://example.com --authorized-by ./authorization.toml
```

## 5. Review

```bash
bugraiser review
```

A terminal UI walks through each finding. For every one you can **approve**, **edit**, **mark duplicate / false positive**, or **reject**. Work through the verification checklist in `docs/DISCLOSURE_POLICY.md`.

## 6. Raise a report

Always preview first:

```bash
bugraiser raise --finding F-001 --dry-run
```

The dry run shows the rendered report, the chosen channel (for example GitHub Private Vulnerability Reporting), and the disclosure deadline. When you are satisfied:

```bash
bugraiser raise --finding F-001
```

Security issues are never posted publicly. If the router cannot find a private channel, it stops and asks you what to do.

## 7. Track and disclose

```bash
bugraiser status                      # acknowledgments and deadlines
bugraiser disclose --finding F-001    # draft the public advisory after the fix
```

## Scanning your own software

```bash
bugraiser scan host --local           # installed packages vs known vulnerabilities
bugraiser scan repo .                 # the project in the current directory
```

## Troubleshooting

| Problem | Try |
|---|---|
| "authorization required" | Provide `--authorized-by`, or check `valid_until` has not passed |
| "target out of scope" | Check `[scope]` include and exclude lists |
| Rate limited by the target | Lower `rate_limit_rps` in the config |
| Missing external tool | Install it, or disable that backend in `config.toml` |
| No private channel found | Find the maintainer contact manually, then use `--channel email --to <address>` |

More in `docs/FAQ.md`.
