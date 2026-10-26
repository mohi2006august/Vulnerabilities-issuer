## Summary
<!-- What does this PR change and why? Link the issue: Fixes #123 -->

## Type
- [ ] Bug fix
- [ ] New scanner / adapter
- [ ] Feature
- [ ] Docs
- [ ] Refactor / chore

## Checklist
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` passes
- [ ] `cargo deny check` passes (if dependencies changed)
- [ ] Tests added or updated
- [ ] Docs updated (and `CHANGELOG.md` under *Unreleased*)
- [ ] Commits are signed off (`git commit -s`)

## Safety checklist (required for scanners, adapters, and core changes)
- [ ] No scan or network request bypasses the authorization gate, scope check, or rate limiter
- [ ] No secrets or personal data are logged or stored unredacted
- [ ] Nothing is submitted to a third party without the human review step
- [ ] No real third-party targets are used in tests (fixtures only)
- [ ] New network behavior is documented (hosts contacted, request rate)

## Notes for reviewers
