# Threat Model

This document lists what can go wrong when BugRaiser is built and used, and what we do about it. It is a living document; update it with every new scanner or adapter.

## 1. What we protect

| Asset | Why it matters |
|---|---|
| **Third-party systems and their users** | The tool must not harm anyone it scans |
| **Unpatched findings** | Leaking them before a fix puts users at risk |
| **User's tokens and credentials** | GitHub, email, and platform access |
| **Evidence and findings store** | May include sensitive snippets |
| **Project reputation** | Spam or abuse gets the tool banned from platforms |

## 2. Actors

| Actor | Intent |
|---|---|
| Well-meaning researcher | Wants to report bugs responsibly (primary user) |
| Careless user | Scans without permission, floods maintainers |
| Malicious user | Uses the tool as an attack aid or an automated spam source |
| Malicious target | A hostile repo or site that attacks the person running the scan |
| Malicious plugin author | Ships a plugin that steals data or attacks others |
| Supply-chain attacker | Compromises a dependency, release, or build |

## 3. Threats and mitigations

| # | Threat | Mitigation | Residual risk |
|---|---|---|---|
| T1 | Scanning targets without permission | Authorization file required for network scans; scope and expiry enforced; audit log; clear legal notice | A determined user can write a false authorization. The gate is an accountability and speed-bump control, not enforcement. |
| T2 | Excess load or accidental DoS | Default 5 req/s per host, per-host concurrency cap, timeouts, kill switch, no stress-testing scanners | Misconfigured limits by the user |
| T3 | Active exploitation or data access | Scanners detect only; no exploit payload library; `web-active` is opt-in and needs explicit authorization | Third-party scanner backends (ZAP, Nuclei) are more aggressive; defaults use non-intrusive profiles |
| T4 | Spam or low-quality mass reports | Mandatory human review, duplicate detection, verification checklist, per-maintainer report cap, no bulk submit command | Users can still file by hand |
| T5 | Premature public disclosure of security findings | Router never posts security findings publicly; fallback posts a detail-free request for a private contact | User overrides; platform misconfiguration |
| T6 | Secrets leaking into logs, evidence, or reports | Redaction layer at the I/O boundary; secrets scanners record type and location only; `redact_secrets` cannot be disabled in release builds | Novel secret formats missed by patterns |
| T7 | Hostile repository attacks the scanner host (path traversal, symlink escape, zip or tar bombs, huge files, malicious build scripts) | Never build or execute target code; safe extraction with path checks; size, depth, and file-count limits; scanner subprocesses run unprivileged and sandboxed | Bugs in third-party scanner parsers |
| T8 | Hostile website attacks the scanner (oversized responses, redirect to internal addresses, hostile content) | Response size and time limits; block redirects to private, loopback, and link-local ranges unless scope allows; strict parsing in safe Rust | Parser bugs in dependencies |
| T9 | Template or content injection in reports (target-controlled text breaking Markdown, or tricking a reader) | Escape all target-derived text; render in code blocks; reviewer sees the final text | Reviewer inattention |
| T10 | Prompt injection, if AI-assisted triage is added | Treat target content as untrusted data; AI never submits; human review remains mandatory; no tool access for the model | Subtle manipulation of summaries |
| T11 | Malicious plugin | Plugins declare network and auth needs; run sandboxed with no inherited credentials; network only via proxy; plugin list is explicit | Sandboxing is best-effort; user installs untrusted code |
| T12 | Token theft or over-privileged tokens | Tokens only via environment or keychain; fine-grained token guidance; tokens never logged or stored in the findings DB | Compromised developer machine |
| T13 | Compromised dependency or release | `cargo-deny` and `cargo-audit` in CI; `Cargo.lock` committed; minimal dependencies; pinned CI actions; signed tags; release checksums and signatures; reproducible-build goal | Zero-day in a dependency |
| T14 | Tampered audit log | Append-only JSONL with hash chaining | Local user can still delete the file |
| T15 | Findings store disclosure | File permissions 0600; optional encryption at rest; clear retention controls | Full-disk compromise |
| T16 | Misleading or wrong findings | Confidence scoring, verification step, "unverified" label, no auto-submit | False positives still occur |

## 4. Trust boundaries

```
 [User] ── trusted ── [BugRaiser core]
                          │
          ┌───────────────┼────────────────────┐
     untrusted:      semi-trusted:         external services:
     target content  plugins / scanners    GitHub, OSV, SMTP
```

All target-derived data is treated as hostile until validated.

## 5. Security requirements for contributors

- No `unsafe` Rust in core crates without a documented justification and review (`#![forbid(unsafe_code)]` by default).
- No `unwrap()` or `expect()` on data derived from targets.
- Fuzz parsers that read target-controlled input (use `cargo-fuzz`).
- Every new scanner or adapter updates this table.

## 6. Out of scope

- Attacks requiring full control of the user's machine.
- Misuse that happens entirely outside the tool.
- Legal risk assessment of specific scans (see `RESPONSIBLE_USE.md`).

## 7. Review schedule

Revisit this model at every minor release and before 1.0 commission an independent review.
