# Disclosure Policy

BugRaiser follows **coordinated vulnerability disclosure (CVD)**. Security issues go to the owner privately first. Public disclosure happens only after a fix or a deadline.

## 1. Classify the finding

| Class | Examples | Channel |
|---|---|---|
| **Security vulnerability** | Injection, auth bypass, RCE, exposed secrets, XSS, SSRF | **Private**, never a public issue |
| **Security hardening** | Missing headers, weak TLS config, outdated dependency with no known exploit path | Private or low-key public issue, per maintainer preference |
| **Non-security bug** | Crash, wrong output, broken link, accessibility problem | Public issue |

When unsure, treat it as a security vulnerability.

## 2. Routing order

BugRaiser looks for the reporting channel in this order:

1. The project's `SECURITY.md`
2. GitHub **Private Vulnerability Reporting** / GitLab confidential issues
3. `/.well-known/security.txt` (RFC 9116) on the website
4. A bug bounty or VDP platform page (HackerOne, Bugcrowd, Intigriti, YesWeHack)
5. Registered security contact: package registry metadata, CNA contact
6. Fallback: a maintainer-only email, or a minimal public issue that says "I have a security report, please provide a private contact" **with no technical detail**

## 3. Timeline (default)

| Day | Action |
|---|---|
| 0 | Send private report |
| 7 | If no acknowledgment, send one follow-up through a different channel |
| 90 | Default public disclosure deadline |
| +14 | Grace extension if the vendor shows active progress |
| Any | Earlier disclosure if the issue is actively exploited, or the fix is released |

Deadlines are configurable per target. Vendor requests for more time are considered in good faith.

## 4. CVE assignment

- If the project has a CNA or uses GitHub Security Advisories, let the maintainer request the CVE.
- Otherwise, request one via MITRE or the relevant CNA after the vendor confirms the issue.

## 5. Public disclosure

After the fix or the deadline, BugRaiser can produce a public advisory (see `REPORT_TEMPLATE.md`). Credit the maintainers' fix and the reporter. Do not publish working exploit code for unpatched issues.

## 6. What a good report avoids

- Threats, deadlines phrased as ultimatums, or bounty demands.
- Mass-sending one report to many unrelated projects.
- Unverified scanner output pasted as a "vulnerability" (see the verification checklist below).

## 7. Verification checklist (required before raising)

- [ ] Reproduced the issue manually or with a deterministic test
- [ ] Confirmed the affected version(s)
- [ ] Checked it is not a duplicate (issues, advisories, CVE/OSV, changelog)
- [ ] Confirmed the target is in scope or open to reports
- [ ] Assessed realistic impact, not just theoretical
- [ ] Removed secrets and personal data from evidence
- [ ] Included a suggested fix or mitigation where possible
