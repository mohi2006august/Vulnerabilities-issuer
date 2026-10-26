# Report Templates

Reports are rendered from these templates. Keep them factual, short, and reproducible.

---

## A. Private Vulnerability Report

**Subject:** `[Security] <short title> in <project> <version>`

```markdown
Hello <project> security team,

I found a potential security issue in <project> and am reporting it privately.

## Summary
<One or two sentences: what is wrong and what an attacker could do.>

## Affected versions
- Confirmed: <version or commit>
- Likely also: <range>, not tested

## Severity (my assessment)
CVSS 4.0: <score> (<vector>) — <Low | Medium | High | Critical>
Preconditions: <auth required? user interaction? network position?>

## Technical details
<Root cause. Link to the vulnerable file and line on the exact commit.>

## Steps to reproduce
1. ...
2. ...
3. Expected: ...
4. Actual: ...

## Proof of concept
<Minimal, non-destructive. Attach as a file if long. No real user data.>

## Impact
<Realistic impact on confidentiality, integrity, availability.>

## Suggested fix
<Patch, diff, or mitigation idea.>

## Disclosure
I follow a 90-day coordinated disclosure timeline from today (<date>), and I'm happy to
adjust if you need more time to ship a fix. Please let me know if you'd like a CVE
requested through your CNA or GitHub Security Advisories.

## Credit
Please credit me as: <name / handle / "anonymous">

Thank you,
<name> — <contact>
Report prepared with BugRaiser v<version>
```

---

## B. Public Bug Report (non-security)

**Title:** `<component>: <what goes wrong> when <condition>`

```markdown
## Description
<Clear summary.>

## Environment
- Version / commit:
- OS:
- Runtime / browser:

## Steps to reproduce
1.
2.
3.

## Expected behavior

## Actual behavior
<Logs, stack trace, screenshots.>

## Possible cause / fix
<Optional pointer.>

## Checklist
- [x] Searched existing issues; this is not a duplicate
- [x] Reproduced on the latest release / main branch
```

---

## C. Website Report (to a site owner)

```markdown
Hello,

While reviewing <site> within your published scope (<link to policy or authorization>),
I noticed the following:

## Finding
<Title>

## Location
<URL / endpoint / parameter>

## Details
<What was observed, with timestamps and request/response excerpts (secrets redacted).>

## Risk
<Plain-language impact.>

## Recommendation
<Specific fix.>

I did not access, modify, or retain any user data. I'm happy to retest after a fix.

— <name>, <contact>
```

---

## D. Public Advisory (after fix or deadline)

```markdown
# <Project> — <Title> (<CVE or GHSA id>)

- **Severity:** <score / rating>
- **Affected:** <versions>
- **Fixed in:** <version>
- **Reported:** <date> | **Fixed:** <date> | **Disclosed:** <date>

## Description
## Impact
## Workarounds
## Fix
## Timeline
## Credits
```

---

## Writing rules

1. Lead with impact, then details.
2. One issue per report.
3. Exact versions and commit hashes.
4. Never include real personal data or live credentials.
5. Be polite. Maintainers are usually volunteers.
6. No pressure, threats, or payment demands.
