# Changelog

All notable changes are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project adheres to
[Semantic Versioning](https://semver.org/). Before 1.0, minor versions may contain breaking changes.

## [Unreleased]

### Added
- Initial documentation set: README, responsible use, disclosure policy, architecture,
  report templates, configuration, plugin guide, threat model, privacy, FAQ, governance.
- Dual license: MIT OR Apache-2.0.

## Release process

1. Move *Unreleased* entries under a new version heading with the date.
2. Bump versions in the workspace `Cargo.toml` and run `cargo update -w`.
3. Tag `vX.Y.Z` (signed) and push; CI builds release binaries and publishes to crates.io.
4. Publish GitHub release notes copied from this file.
5. Security fixes: release the fix first, then publish the advisory (see `docs/DISCLOSURE_POLICY.md`).

[Unreleased]: https://github.com/<org>/bugraiser/compare/HEAD...HEAD
