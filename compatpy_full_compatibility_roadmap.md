# CompatPy: Full Compatibility Roadmap

**Goal:** Make CompatPy a robust Python compatibility analyzer that
evaluates dependency requirements, environment markers, package
metadata, and dependency conflicts across Python versions.

This roadmap is intentionally comprehensive. Some items are already
implemented and should be audited with tests rather than rewritten
automatically.

/ = partially implemented or incompletely verified
[ ] = not verified if working on project

## Phase 1: Requirements-file parsing

### Basic requirement parsing

-   [x] Parse and normalize package names.
-   [x] Parse `==`, `!=`, `<`, `<=`, `>`, `>=`, `~=`, and wildcard
    specifiers.
-   [x] Support requirements without version specifiers.
-   [/] Support comma-separated specifiers.
-   [x] Ignore blank lines and full-line comments.
-   [ ] Handle inline comments without stripping valid URL fragments.
-   [/] Handle whitespace and line-ending differences.
-   [ ] Handle supported line continuations.

### Requirement syntax

-   [/] Parse extras, such as `requests[security]`.
-   [ ] Parse direct URL requirements.
-   [ ] Handle Git/VCS references where supported.
-   [ ] Handle local paths and editable installs where supported.
-   [/] Recognize valid package-name punctuation.
-   [ ] Support standard PEP 508 requirement syntax.
-   [ ] Report malformed requirements clearly.

### File structure

-   [ ] Recognize `-r` / `--requirement` and `-c` / `--constraint`.
-   [ ] Resolve nested paths relative to the containing file.
-   [ ] Support nested requirements and constraint files.
-   [ ] Detect recursive include loops.
-   [ ] Report missing or unreadable included files.
-   [/] Keep comments and annotations separate from dependency
    semantics.

## Phase 2: Version specifiers and environment markers

### Version specifiers

-   [/] Audit exact versions, inequalities, and comma-separated specifiers.
-   [/] Audit compatible-release operator `~=`.
-   [/] Audit wildcard equality `==3.12.*`.
-   [/] Audit wildcard exclusion `!=3.12.*`.
-   [/] Audit post-release ordering.
-   [ ] Support development releases, such as `1.0.dev1`.
-   [/] Support alpha and beta releases, such as `1.0a1` and `1.0b2`.
-   [/] Support release candidates, such as `1.0rc1`.
-   [ ] Support epochs, such as `1!2.0`.
-   [ ] Support local versions, such as `1.0+vendor.1`.
-   [ ] Implement PEP 440 normalization and equivalence rules.
-   [/] Handle invalid version strings without panics.

### Marker parsing

-   [/] Parse marker expressions after semicolons.
-   [ ] Support `python_version` and `python_full_version`.
-   [ ] Support parentheses and boolean grouping.
-   [ ] Implement correct `and` / `or` precedence.
-   [ ] Support `==`, `!=`, `<`, `<=`, `>`, `>=`, `in`, and `not in` where valid.
-   [ ] Support the `extra` marker variable.
-   [ ] Support `implementation_name` and `implementation_version`.
-   [ ] Support `os_name`, `sys_platform`, `platform_system`, `platform_machine`, and `platform_python_implementation`.
-   [ ] Support other standardized PEP 508 marker variables.
-   [ ] Handle unknown variables and invalid expressions safely.

### Marker evaluation

-   [ ] Evaluate markers independently for each candidate Python version.
-   [ ] Skip requirements whose markers evaluate false.
-   [ ] Include requirements whose markers evaluate true.
-   [ ] Handle markers combining Python and operating-system conditions.
-   [ ] Evaluate extra-specific markers in the relevant extras context.
-   [ ] Test version boundaries such as Python 3.9.9 versus 3.10.0.
-   [ ] Compare version values semantically, not lexicographically.

## Phase 3: Constraints and dependency sources

### Requirements and constraints

-   [ ] Read `-r` / `--requirement` includes.
-   [ ] Read `-c` / `--constraint` includes.
-   [ ] Apply constraints without treating them as ordinary requested
    dependencies.
-   [ ] Combine multiple constraints for the same package.
-   [ ] Detect contradictory requirements and constraints.
-   [ ] Apply environment markers correctly to requirements and
    constraints.
-   [/] Handle repeated package entries.
-   [/] Preserve provenance: source file and line number for each rule.

### Project metadata

-   [x] Read `pyproject.toml` project dependencies.
-   [ ] Read optional-dependency groups from `pyproject.toml`.
-   [ ] Read PEP 621 `requires-python`.
-   [ ] Read `setup.cfg` metadata where practical.
-   [ ] Read `setup.py` metadata where practical without executing
    arbitrary project code.
-   [ ] Read Poetry dependency declarations where practical.
-   [ ] Read PDM and other common project metadata where practical.
-   [/] Support `requirements.in` and generated `requirements.txt`
    workflows.
-   [/] Clearly report which sources were analyzed or skipped.

### Package identity

-   [ ] Normalize package names according to Python packaging rules.
-   [ ] Merge equivalent names such as `my_package` and `my-package`.
-   [ ] Distinguish extras from separate package names.
-   [ ] Merge duplicate dependencies from multiple files appropriately.
-   [/] Track direct-reference URLs and indexes where relevant.
-   [/] Avoid silently merging genuinely different direct references.

## Phase 4: Python version discovery and project ranges

### Candidate versions

-   [ ] Discover Python versions from project metadata when specified.
-   [ ] Support a configurable candidate-version range.
-   [ ] Include newly released Python versions when reliable metadata is
    available.
-   [/] Distinguish released versions from prereleases.
-   [/] Define a policy for minor-version versus patch-version
    evaluation.
-   [/] Correctly handle versions such as 3.9 through 3.14.
-   [ ] Avoid relying forever on an obsolete hardcoded version list.

### Project-level restrictions

-   [ ] Read `requires-python` from project metadata.
-   [ ] Intersect it with dependency compatibility restrictions.
-   [/] Handle lower and upper bounds correctly.
-   [ ] Represent disjoint compatible ranges if needed.
-   [ ] Explain why a Python version was excluded.
-   [ ] Distinguish explicit project restrictions from dependency
    conflicts.

### Results

-   [x] Report compatible candidate Python versions.
-   [ ] Report incompatible candidates separately.
-   [/] Define behavior when no candidate is compatible.
-   [/] Define behavior when metadata is incomplete.
-   [ ] Explain the scope of the result: declared compatibility versus
    guaranteed runtime success.

## Phase 5: PyPI and package release metadata

### Release metadata

-   [x] Retrieve package release metadata from PyPI.
-   [x] Read each release's `Requires-Python` field.
-   [ ] Reject releases whose declared Python range excludes the
    candidate.
-   [/] Handle missing or malformed metadata.
-   [ ] Handle yanked releases appropriately.
-   [/] Define prerelease and development-release selection policy.
-   [ ] Cache metadata to reduce network requests.
-   [/] Handle network failures, rate limits, and timeouts.
-   [ ] Support offline analysis from available cache.
-   [ ] Support configurable indexes or sources where feasible.

### Candidate releases

-   [/] Find releases satisfying all applicable specifiers.
-   [ ] Combine release metadata with marker evaluation.
-   [/] Do not treat a matching version number as proof of Python
    compatibility.
-   [/] Detect packages with no compatible release.
-   [ ] Explain whether failure comes from a version range or Python
    support.
-   [ ] Define behavior for direct URLs and local packages with
    unavailable metadata.

### Metadata reliability

-   [/] Record metadata provenance.
-   [ ] Record cache/fetch timestamps where useful.
-   [/] Do not treat missing metadata as proof of compatibility.
-   [ ] Distinguish known incompatibility from unknown compatibility.
-   [ ] Consider platform-specific wheels and source distributions as a
    later capability.

## Phase 6: Dependency conflicts and resolution

### Conflict detection

-   [ ] Detect incompatible version ranges for the same package.
-   [ ] Detect contradictions between project requirements and
    constraints.
-   [ ] Distinguish missing metadata from a real conflict.
-   [/] Report the origins of conflicting requirements.
-   [ ] Handle marker-dependent requirements for the same package.
-   [ ] Avoid false conflicts between mutually exclusive environment
    branches.

### Transitive dependencies

-   [ ] Read dependencies declared by candidate package releases.
-   [ ] Evaluate dependency markers recursively.
-   [ ] Propagate Python restrictions through the dependency tree.
-   [ ] Combine transitive version constraints.
-   [ ] Detect dependency cycles safely.
-   [ ] Handle extras and their additional dependencies.
-   [ ] Include optional dependencies only when requested or configured.

### Resolution strategy

-   [/] Choose candidate releases using a documented strategy.
-   [ ] Backtrack when a selected release creates a conflict.
-   [/] Do not assume the newest release is always correct.
-   [/] Define prerelease selection behavior.
-   [ ] Report unresolved dependency sets clearly.
-   [/] Keep results deterministic for unchanged inputs and metadata.

## Phase 7: Real-world regression testing

### Regression fixtures

-   [/] Test the supplied uv-generated requirements file.
-   [ ] Test repeated `# via` annotations.
-   [ ] Test multiple Python marker branches.
-   [ ] Test separate common constraint files.
-   [ ] Test nested `-r` and `-c` references.
-   [/] Test exact pins mixed with bounded ranges.
-   [ ] Test extras and optional dependencies.
-   [/] Test old and new package-version formats.

### Edge cases

-   [/] Test Python-version boundaries.
-   [ ] Test marker combinations using `and`, `or`, and parentheses.
-   [ ] Test conflicting dependencies and diagnostic output.
-   [ ] Test packages without a release for a candidate Python version.
-   [/] Test empty files and projects without dependencies.
-   [/] Test malformed files and unreadable paths.
-   [/] Test duplicate and normalized package names.
-   [ ] Test platform markers and unavailable metadata.

### Reference validation

-   [ ] Compare selected outcomes with pip or uv where feasible.
-   [/] Create fixtures with known expected outcomes.
-   [ ] Document deliberate differences from pip or uv.
-   [/] Prevent regressions with automated tests.

## Phase 8: Diagnostics, CLI, performance, and resilience

### Explanations

-   [ ] Identify the package responsible for excluding a Python version.
-   [/] Show the relevant version specifier and marker.
-   [/] Show source file and line number where possible.
-   [ ] Explain conflicts between requirements.
-   [ ] Distinguish compatible, incompatible, and unknown outcomes.
-   [ ] Warn about missing or stale metadata.
-   [ ] Summarize why each candidate passed or failed.

### CLI behavior

-   [/] Keep the existing command-line interface predictable.
-   [/] Validate project-directory arguments.
-   [/] Report invalid paths clearly.
-   [/] Define exit codes for success, incompatibility, and analysis
    errors.
-   [ ] Keep stdout suitable for piping.
-   [/] Send diagnostics to stderr where appropriate.
-   [ ] Consider machine-readable JSON output.
-   [/] Consider verbose/debug output.

### Performance and resilience

-   [ ] Avoid fetching identical package metadata repeatedly.
-   [ ] Cache repeated marker and version calculations where useful.
-   [/] Handle large requirements files efficiently.
-   [/] Handle interrupted network requests gracefully.
-   [/] Avoid panics on user-controlled input.
-   [/] Keep errors contextual and actionable.

## Phase 9: Rust code quality and release process

### Code structure

-   [/] Separate parsing from evaluation.
-   [/] Separate version parsing from comparison.
-   [ ] Separate marker parsing from evaluation.
-   [/] Separate file loading from dependency resolution.
-   [/] Use explicit types for requirements, markers, constraints, and
    results.
-   [/] Keep public APIs stable where practical.
-   [ ] Document tricky PEP 440 and PEP 508 behavior.

### Quality gates

-   [/] Run `cargo fmt`.
-   [/] Run `cargo test`.
-   [ ] Run `cargo clippy` and address relevant warnings.
-   [/] Add unit tests for every new behavior.
-   [ ] Add integration tests using temporary project directories.
-   [/] Test Windows and Unix path behavior.
-   [/] Avoid unnecessary dependencies and excessive network calls.
-   [/] Review panic-prone and error-handling paths.

### Release readiness

-   [/] Update README examples and supported-feature documentation.
-   [/] Update `CHANGELOG.md`.
-   [/] Review Cargo.toml metadata and dependency versions.
-   [/] Check license and attribution files.
-   [/] Build release binaries for supported targets.
-   [/] Verify `cargo package` and publishing readiness.
-   [/] Create a Git commit for every successful implementation or fix.
-   [/] Release a new version only when the project is ready.

## Phase 10: Advanced capabilities

### Platform and interpreter support

-   [ ] Evaluate operating-system-specific markers.
-   [ ] Distinguish CPython from alternative implementations where
    relevant.
-   [ ] Consider architecture-specific wheel availability.
-   [ ] Consider ABI and platform tags.
-   [/] Distinguish metadata-level compatibility from installability on
    a specific machine.

### Project ecosystems

-   [/] Improve support for common `pyproject.toml` configurations.
-   [ ] Improve Poetry, PDM, and other ecosystem-specific parsing.
-   [ ] Handle documented lockfile formats.
-   [ ] Explain differences between ecosystem resolution rules and
    standard requirements syntax.

### Developer integrations

-   [ ] Add JSON output for CI pipelines.
-   [/] Provide stable exit codes for automation.
-   [ ] Consider GitHub Actions documentation.
-   [/] Consider a library API alongside the CLI.
-   [ ] Consider reports comparing Python versions.
-   [ ] Consider analysis of a specific target environment.

------------------------------------------------------------------------

## Recommended implementation order

1.  **Audit version semantics:** test `~=`, wildcard specifiers,
    post-releases, prereleases, development releases, epochs, and local
    versions.
2.  **Implement environment markers:** parse the expression after `;`
    and evaluate it for each candidate Python version.
3.  **Implement requirements and constraint includes:** support `-r` and
    `-c`, relative paths, and correct constraint semantics.
4.  **Integrate PyPI release metadata:** use `Requires-Python` to
    improve release selection.
5.  **Analyze transitive dependencies:** build the dependency graph,
    propagate constraints, and identify conflicts.
6.  **Test against real projects:** use the supplied uv-generated
    requirements file as a regression fixture and compare with
    established tools.
7.  **Improve reporting and reliability:** diagnostics, caching, error
    handling, and automation-friendly output.

## Rules for every implementation

-   Make one focused change at a time.
-   Add tests for the behavior being introduced.
-   Run `cargo fmt`, `cargo test`, and relevant quality checks.
-   Fix regressions before moving on.
-   Commit after every successful implementation or fix, with a clear
    commit message.
-   Keep the real-world requirements file as a long-term regression
    test.
-   Never claim full compatibility when only declared metadata has been
    checked.

## Important scope distinction

CompatPy can determine whether declared requirements and package
metadata **indicate** compatibility. Guaranteeing that every dependency
installs and runs on a given Python version is a larger task involving
build systems, wheels, operating systems, and potentially actual
installations.

**Immediate next task:** implement and test environment markers. Then
handle nested requirements and constraints, followed by release metadata
and deeper dependency resolution.
