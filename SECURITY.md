# Security Policy

BugRaiser handles vulnerability data and credentials, so we take its own security seriously.

## Supported versions

| Version | Supported |
|---|---|
| `main` | Yes |
| Latest release | Yes |
| Older releases | No |

## Reporting a vulnerability in BugRaiser

**Please do not open a public issue.**

Use one of these private channels:

1. GitHub: *Security* tab → *Report a vulnerability* (preferred)
2. Email: `security@your-domain.example` (PGP key: `<fingerprint / link>`)

Include a description, affected version, reproduction steps, and your impact assessment.

## What to expect

| Step | Target time |
|---|---|
| Acknowledgment | 3 business days |
| Initial assessment | 7 days |
| Fix or mitigation plan | 30 days for high/critical, 90 days otherwise |
| Public advisory | After a fix is released |

We credit reporters unless you ask us not to.

## Scope

In scope:
- Bypass of the authorization gate, rate limiter, or human-review step
- Secret leakage into logs, reports, or the findings store
- Injection or code execution through scanner output, templates, or plugin loading
- Unsafe handling of tokens or credentials
- Report-routing bugs that could send sensitive details to a public channel

Out of scope:
- Issues requiring a malicious local user who already controls your machine
- Vulnerabilities in third-party scanners (report them upstream)
- Social engineering

## Safe harbor

We will not pursue legal action against researchers who act in good faith, stay within this policy, avoid harming others' data or systems, and give us reasonable time to fix issues.
