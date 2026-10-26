# FAQ

## General

**What is BugRaiser?**
An open-source Rust tool that finds vulnerabilities and bugs, verifies them, checks for duplicates, and helps you report them responsibly to the right owner.

**Does it hack things?**
No. It detects and confirms issues with minimal, non-destructive checks. It does not ship exploit payloads or try to gain access.

**Does it report automatically?**
No. A human must review and approve every report. There is no bulk-submit command, by design.

**Why Rust?**
A single static binary that is easy to install, memory safety for a tool that parses hostile input, and good performance for concurrent, rate-limited scanning.

**What license is it under?**
MIT OR Apache-2.0, at your choice. See `LICENSE-MIT` and `LICENSE-APACHE`.

## Legal and ethical

**Can I scan any website?**
No. Only sites you own, or where you have written permission or a published scope (bug bounty or VDP) that covers your activity. See `RESPONSIBLE_USE.md`.

**Is scanning open-source code allowed?**
Reading public source code and analyzing a clone is fine. Testing a live deployment of that project is not, unless authorized.

**I found something serious. What now?**
Stop, do not exploit further, and report privately. See `DISCLOSURE_POLICY.md`.

**Can I get a bounty?**
Only through programs that offer one and whose rules you follow. Never demand payment or threaten disclosure.

**Am I legally safe if I follow these docs?**
These docs reduce risk but are not legal advice. Laws differ by country. Get permission first.

## Usage

**Why was my finding marked duplicate?**
The deduplicator found a matching CVE, advisory, or open issue. Check the linked references. If you think it is different, edit the finding and explain why.

**The router couldn't find a private channel. What now?**
Look for a maintainer contact manually. If none exists, file a minimal public issue asking for a private contact, with no technical details.

**Can I run it in CI?**
Yes, for your own repositories (`scan repo .` with `--dry-run` for reporting). Do not run network scans of third-party sites from CI.

**Does it work offline?**
Partly. Use `--offline` with a local vulnerability database. Submission is disabled.

**It missed a bug I know exists.**
Scanners have limits. Open an issue with a safe reproduction (a fixture, not a live target) so we can improve coverage.

## Contributing

**How do I add a scanner?**
See `PLUGIN_GUIDE.md`.

**Can I add an exploit module?**
No. See `CONTRIBUTING.md`: weaponized exploits are rejected.

**Where do I report a vulnerability in BugRaiser?**
Privately, via `SECURITY.md`.
