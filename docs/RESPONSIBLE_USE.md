# Responsible Use Policy

**Everyone using BugRaiser must read and follow this document.** Running the tool means you accept it.

## 1. Authorization comes first

| Target type | What you need before scanning |
|---|---|
| Open-source repository (code only) | Nothing for reading public code. Static analysis of source you have cloned is fine. |
| Open-source project's live deployment | Written permission, or a published bug bounty / VDP that covers it |
| Software installed on your own machine | You own it. Fine. |
| Websites and web apps | **Written authorization from the owner**, or a published scope (bug bounty, `security.txt` policy, VDP) that covers your activity |
| Anyone else's systems | Do not scan |

BugRaiser requires an **authorization file** for any network-facing scan:

```toml
# authorization.toml
target = "example.com"
authorized_by = "Jane Doe, CTO"
contact = "security@example.com"
valid_from = 2026-10-26
valid_until = 2026-11-26
evidence = "https://example.com/security-policy"   # or a signed PDF reference

[scope]
include = ["example.com", "*.example.com"]
exclude = ["payments.example.com"]
```

The tool refuses to run network scans without it. Do not circumvent this check.

## 2. Allowed activity

- Passive and low-impact checks: headers, TLS configuration, exposed version strings, known-CVE matching, public file exposure (`/.git`, `.env` presence checks without reading secrets).
- Static analysis of source code you can lawfully access.
- Dependency and SBOM auditing.
- Minimal proof of concept that demonstrates the issue without causing harm.

## 3. Prohibited activity

- Exploiting a vulnerability beyond the minimum needed to confirm it.
- Accessing, copying, modifying, or deleting data that is not yours.
- Denial of service, load or stress testing, brute-forcing, credential stuffing.
- Social engineering, phishing, or physical attacks.
- Testing third-party services a target depends on (CDNs, payment processors, SSO providers) unless they are explicitly in scope.
- Publishing a vulnerability before the disclosure window ends.
- Selling or trading findings.

## 4. Handling sensitive data

If you encounter personal data, credentials, or secrets:

1. Stop immediately.
2. Do not store or copy them. BugRaiser redacts secrets in logs by default.
3. Report the exposure, not the contents.

## 5. Rate limits and politeness

Default cap: **5 requests per second per host**, with `User-Agent: BugRaiser/<version> (+https://your-project-url)`. Honor `robots.txt` for crawling unless the owner authorized otherwise.

## 6. Legal notice

Laws on unauthorized computer access differ by country (CFAA in the US, Computer Misuse Act in the UK, IT Act 2000 in India, and others). Good intentions are not a legal defense. Get permission first. This project provides no legal advice. If unsure, ask a lawyer or the target's security contact.

## 7. Misuse

Maintainers may remove contributions, ban accounts, and publicly state that a user violated this policy. Reports of misuse go to `conduct@your-domain.example`.
