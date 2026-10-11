# Changelog

---

## 26.10.2 - 2026-10-10

### Added
- Support Git and VCS direct references, including editable VCS requirements.
- Support direct URL dependencies.
- Add Python 3.15 to the compatibility range.
- Support package extras.

### Fixed
- Fix `--version` and `--help` CLI flags.
- Expand requirements discovery and VCS tests.

---

## 26.10.1 - 2026-10-10

### Fixed
- Fix Linux ARM64 release builds and cross-compilation configuration.
- Improve OpenSSL configuration and diagnostics for ARM64 builds.
- Fix release workflow issues affecting binary builds.

### Changed
- Update GitHub Actions checkout to Node.js 24.

---

## 1.0.0 - 2026-10-08

### Added

- Discover Python dependencies from `requirements.txt`.
- Discover Python dependencies from `pyproject.toml`.
- Retrieve package metadata from PyPI.
- Resolve package versions against dependency specifiers.
- Determine Python versions supported by project dependencies.
- Exclude explicitly incompatible Python versions.
- Prefer stable package releases over prereleases.
- Recommend compatible Python versions.
