# Privacy and Data Handling

> This is a technical description of what BugRaiser does with data. It is not legal advice. If you operate it for an organization, check your obligations under laws such as GDPR or India's DPDP Act.

## 1. Telemetry

**None.** BugRaiser sends no analytics, crash reports, or usage statistics. Any future opt-in telemetry would be off by default and documented here first.

## 2. What is stored locally

| Data | Where | Notes |
|---|---|---|
| Config | `~/.config/bugraiser/config.toml` | Your name and contact email; no secrets |
| Findings and evidence | `~/.local/share/bugraiser/` | Created with permissions `0600`; redacted before writing |
| Audit log | `~/.local/share/bugraiser/audit.jsonl` | Actions, targets, timestamps, authorization references |
| Cloned repositories | Temporary work directory | Deleted after the scan unless `--keep-work` is set |

Paths follow platform conventions on Windows and macOS.

## 3. What leaves your machine

| Destination | What is sent | When |
|---|---|---|
| Target (repo host or website) | Normal requests with the identified `User-Agent` | During scans |
| OSV, NVD, GitHub Advisory DB | Package names and versions (and sometimes hashes) | Dependency checks and deduplication |
| GitHub / GitLab APIs | Search queries for duplicates; the report when you approve it | Deduplication and submission |
| SMTP server | The report email you approved | Submission by email |
| Anything else | Nothing | Never |

Offline mode (`--offline`) uses a local vulnerability database snapshot and disables submission.

## 4. Secrets and personal data

- Secrets found in a target are **not stored**. Findings keep the type and location only.
- The redaction layer masks tokens, keys, passwords, and email addresses in logs and evidence.
- If you find personal data in a target, stop, do not collect it, and report the exposure only. See `RESPONSIBLE_USE.md`.

## 5. Your identity in reports

Reports include the name, handle, and contact you configured. Choose `credit_preference = "anonymous"` to omit your name. Remember that email headers and platform accounts can still identify you.

## 6. Retention and deletion

```bash
bugraiser data purge --older-than 180d      # remove old findings and evidence
bugraiser data purge --finding F-001        # remove one finding
bugraiser data purge --all                  # remove everything
```

Default retention is until you delete it. The audit log is kept separately; purge it explicitly if your policy allows.

## 7. Third-party data in this project

The project repository must never contain real vulnerabilities in unpatched software, real credentials, or real personal data. Test fixtures use synthetic data only.
