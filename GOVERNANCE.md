# Governance

BugRaiser is a community project. This document explains who decides what.

## Roles

| Role | Who | Can do |
|---|---|---|
| **User** | Anyone | Use the tool, file issues, join discussions |
| **Contributor** | Anyone with a merged change | Propose changes, review others' PRs |
| **Maintainer** | Listed in `MAINTAINERS.md` | Merge PRs, cut releases, triage, moderate |
| **Safety owners** | Subset of maintainers | Approve changes to safety-critical code (see below) |

Initial maintainers: `<your name / handle>` (add others as the project grows).

## Safety-critical code

These areas protect third parties and the project's reputation. Changes need **two maintainer approvals**, at least one of them a safety owner:

- authorization gate, scope checking, rate limiting (`bugraiser-core/src/safety/`)
- secret redaction
- disclosure routing (`bugraiser-routers`)
- the human-review step and anything that submits to external services (`bugraiser-adapters`)
- plugin sandboxing and loading

Enforce with `.github/CODEOWNERS`.

## Decision making

1. **Lazy consensus.** A proposal with no objection after 5 days (72 hours for small fixes) is accepted.
2. **Objections** must state a concrete concern. Discuss on the issue or PR.
3. **No consensus:** maintainers vote. Simple majority; ties are decided by the project lead.
4. **Large changes** (new scanner category, plugin API change, license change) need an RFC: a PR adding `docs/rfcs/NNNN-title.md`, open for at least 14 days.
5. **License changes** require agreement from all maintainers and notice to contributors.

## Becoming a maintainer

- Sustained, quality contributions over roughly 3 months.
- Demonstrated understanding of the safety model.
- Nominated by a maintainer and approved by a majority, with no unresolved objection.

Maintainers who are inactive for 6 months are moved to *emeritus* status and can return on request.

## Removing access

Maintainers may be removed for repeated Code of Conduct violations, breaching the Responsible Use Policy, or bypassing safety controls. Decided by a majority of the other maintainers.

## Releases

Any maintainer can cut a release following the process in `CHANGELOG.md`. Releases containing safety changes need a safety owner's sign-off.

## Trademark and name

The project name and logo are used to identify the official project. Forks are welcome under the license but should use a different name.

## Changing this document

By PR, with the same rules as an RFC.
