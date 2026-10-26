# Local API

The first backend slice stores findings in SQLite and exposes a loopback-only HTTP API. It does not scan targets or submit reports.

Run it with `cargo run`. It listens on `127.0.0.1:3000` by default. Set `BUGRAISER_PORT` to change the port and `BUGRAISER_DATABASE` to choose the SQLite file path.

All routes are under `/api/v1`:

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/health` | Liveness check |
| `GET` | `/findings` | List findings |
| `POST` | `/findings` | Store a finding in `needs-review` |
| `POST` | `/findings/{id}/decision` | Review once: `approved`, `rejected`, `duplicate`, or `false-positive` |

Create a finding with JSON fields `title`, `target`, `scanner`, and optional `component` and `severity`. Request bodies are limited to 64 KiB and each text field to 2,000 characters. Review decisions are recorded with the creation event in the local SQLite audit log. Approval only changes the finding status; no report is sent.

Example:

```sh
curl -X POST http://127.0.0.1:3000/api/v1/findings \
  -H 'content-type: application/json' \
  -d '{"title":"Example issue","target":"local/project","scanner":"manual"}'
```
