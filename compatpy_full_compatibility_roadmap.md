# CompatPy: Full Compatibility Roadmap

**Goal:** Make CompatPy a robust Python compatibility analyzer that
evaluates dependency requirements, environment markers, package
metadata, and dependency conflicts across Python versions.

This roadmap is intentionally comprehensive. Some items are already
implemented and should be audited with tests rather than rewritten
automatically.

X = not implemented
/ = partially implemented or incompletely verified
Y = implemented with direct evidence
[ ] = not verified if working on project

## Phase 1: Requirements-file parsing

### Basic requirement parsing

-   [Y] Parse and normalize package names.
-   [/] Parse `==`, `!=`, `<`, `<=`, `>`, `>=`, `~=`, and wildcard
    specifiers.
-   [Y] Support requirements without version specifiers.
-   [/] Support comma-separated specifiers.
-   [Y] Ignore blank lines and full-line comments.
-   [ ] Handle inline comments without stripping valid URL fragments.
-   [/] Handle whitespace and line-ending differences.
-   [X] Handle supported line continuations.

### Requirement syntax

-   [/] Parse extras, such as `requests[security]`.
-   [X] Parse direct URL requirements.
-   [X] Handle Git/VCS references where supported.
-   [X] Handle local paths and editable installs where supported.
-   [/] Recognize valid package-name punctuation.
-   [X] Support standard PEP 508 requirement syntax.
-   [X] Report malformed requirements clearly.

### File structure

-   [X] Recognize `-r` / `--requirement` and `-c` / `--constraint`.
-   [X] Resolve nested paths relative to the containing file.
-   [X] Support nested requirements and constraint files.
-   [X] Detect recursive include loops.
-   [X] Report missing or unreadable included files.
-   [/] Keep comments and annotations separate from dependency
    semantics.

## Phase 2: Version specifiers and environment markers

### Version specifiers

-   [/] Audit exact versions, inequalities, and comma-separated specifiers.
-   [/] Audit compatible-release operator `~=`.
-   [/] Audit wildcard equality `==3.12.*`.
-   [/] Audit wildcard exclusion `!=3.12.*`.
-   [/] Audit post-release ordering.
-   [X] Support development releases, such as `1.0.dev1`.
-   [/] Support alpha and beta releases, such as `1.0a1` and `1.0b2`.
-   [/] Support release candidates, such as `1.0rc1`.
-   [X] Support epochs, such as `1!2.0`.
-   [X] Support local versions, such as `1.0+vendor.1`.
-   [X] Implement PEP 440 normalization and equivalence rules.
-   [/] Handle invalid version strings without panics.

### Marker parsing

-   [/] Parse marker expressions after semicolons.
-   [X] Support `python_version` and `python_full_version`.
-   [X] Support parentheses and boolean grouping.
-   [X] Implement correct `and` / `or` precedence.
-   [X] Support `==`, `!=`, `<`, `<=`, `>`, `>=`, `in`, and `not in` where valid.
-   [X] Support the `extra` marker variable.
-   [X] Support `implementation_name` and `implementation_version`.
-   [X] Support `os_name`, `sys_platform`, `platform_system`, `platform_machine`, and `platform_python_implementation`.
-   [X] Support other standardized PEP 508 marker variables.
-   [X] Handle unknown variables and invalid expressions safely.

### Marker evaluation

-   [X] Evaluate markers independently for each candidate Python version.
-   [X] Skip requirements whose markers evaluate false.
-   [X] Include requirements whose markers evaluate true.
-   [X] Handle markers combining Python and operating-system conditions.
-   [X] Evaluate extra-specific markers in the relevant extras context.
-   [X] Test version boundaries such as Python 3.9.9 versus 3.10.0.
-   [X] Compare version values semantically, not lexicographically.

## Phase 3: Constraints and dependency sources

### Requirements and constraints

-   [X] Read `-r` / `--requirement` includes.
-   [X] Read `-c` / `--constraint` includes.
-   [X] Apply constraints without treating them as ordinary requested
    dependencies.
-   [X] Combine multiple constraints for the same package.
-   [X] Detect contradictory requirements and constraints.
-   [X] Apply environment markers correctly to requirements and
    constraints.
-   [/] Handle repeated package entries.
-   [/] Preserve provenance: source file and line number for each rule.

### Project metadata

-   [Y] Read `pyproject.toml` project dependencies.
-   [X] Read optional-dependency groups from `pyproject.toml`.
-   [X] Read PEP 621 `requires-python`.
-   [X] Read `setup.cfg` metadata where practical.
-   [X] Read `setup.py` metadata where practical without executing
    arbitrary project code.
-   [X] Read Poetry dependency declarations where practical.
-   [X] Read PDM and other common project metadata where practical.
-   [/] Support `requirements.in` and generated `requirements.txt`
    workflows.
-   [/] Clearly report which sources were analyzed or skipped.

### Package identity

-   [X] Normalize package names according to Python packaging rules.
-   [X] Merge equivalent names such as `my_package` and `my-package`.
-   [X] Distinguish extras from separate package names.
-   [X] Merge duplicate dependencies from multiple files appropriately.
-   [/] Track direct-reference URLs and indexes where relevant.
-   [/] Avoid silently merging genuinely different direct references.

## Phase 4: Python version discovery and project ranges

### Candidate versions

-   [X] Discover Python versions from project metadata when specified.
-   [X] Support a configurable candidate-version range.
-   [X] Include newly released Python versions when reliable metadata is
    available.
-   [/] Distinguish released versions from prereleases.
-   [/] Define a policy for minor-version versus patch-version
    evaluation.
-   [/] Correctly handle versions such as 3.9 through 3.14.
-   [X] Avoid relying forever on an obsolete hardcoded version list.

### Project-level restrictions

-   [X] Read `requires-python` from project metadata.
-   [X] Intersect it with dependency compatibility restrictions.
-   [/] Handle lower and upper bounds correctly.
-   [X] Represent disjoint compatible ranges if needed.
-   [X] Explain why a Python version was excluded.
-   [X] Distinguish explicit project restrictions from dependency
    conflicts.

### Results

-   [Y] Report compatible candidate Python versions.
-   [X] Report incompatible candidates separately.
-   [/] Define behavior when no candidate is compatible.
-   [/] Define behavior when metadata is incomplete.
-   [X] Explain the scope of the result: declared compatibility versus
    guaranteed runtime success.

## Phase 5: PyPI and package release metadata

### Release metadata

-   [Y] Retrieve package release metadata from PyPI.
-   [Y] Read each release's `Requires-Python` field.
-   [X] Reject releases whose declared Python range excludes the
    candidate.
-   [/] Handle missing or malformed metadata.
-   [X] Handle yanked releases appropriately.
-   [/] Define prerelease and development-release selection policy.
-   [X] Cache metadata to reduce network requests.
-   [/] Handle network failures, rate limits, and timeouts.
-   [X] Support offline analysis from available cache.
-   [X] Support configurable indexes or sources where feasible.

### Candidate releases

-   [/] Find releases satisfying all applicable specifiers.
-   [X] Combine release metadata with marker evaluation.
-   [/] Do not treat a matching version number as proof of Python
    compatibility.
-   [/] Detect packages with no compatible release.
-   [X] Explain whether failure comes from a version range or Python
    support.
-   [X] Define behavior for direct URLs and local packages with
    unavailable metadata.

### Metadata reliability

-   [/] Record metadata provenance.
-   [X] Record cache/fetch timestamps where useful.
-   [/] Do not treat missing metadata as proof of compatibility.
-   [X] Distinguish known incompatibility from unknown compatibility.
-   [ ] Consider platform-specific wheels and source distributions as a
    later capability.

## Phase 6: Dependency conflicts and resolution

### Conflict detection

-   [X] Detect incompatible version ranges for the same package.
-   [X] Detect contradictions between project requirements and
    constraints.
-   [X] Distinguish missing metadata from a real conflict.
-   [/] Report the origins of conflicting requirements.
-   [X] Handle marker-dependent requirements for the same package.
-   [X] Avoid false conflicts between mutually exclusive environment
    branches.

### Transitive dependencies

-   [X] Read dependencies declared by candidate package releases.
-   [X] Evaluate dependency markers recursively.
-   [X] Propagate Python restrictions through the dependency tree.
-   [X] Combine transitive version constraints.
-   [X] Detect dependency cycles safely.
-   [X] Handle extras and their additional dependencies.
-   [X] Include optional dependencies only when requested or configured.

### Resolution strategy

-   [/] Choose candidate releases using a documented strategy.
-   [X] Backtrack when a selected release creates a conflict.
-   [/] Do not assume the newest release is always correct.
-   [/] Define prerelease selection behavior.
-   [X] Report unresolved dependency sets clearly.
-   [/] Keep results deterministic for unchanged inputs and metadata.

## Phase 7: Real-world regression testing

### Regression fixtures

-   [/] Test the supplied uv-generated requirements file.
-   [X] Test repeated `# via` annotations.
-   [X] Test multiple Python marker branches.
-   [X] Test separate common constraint files.
-   [X] Test nested `-r` and `-c` references.
-   [/] Test exact pins mixed with bounded ranges.
-   [X] Test extras and optional dependencies.
-   [/] Test old and new package-version formats.

### Edge cases

-   [/] Test Python-version boundaries.
-   [X] Test marker combinations using `and`, `or`, and parentheses.
-   [X] Test conflicting dependencies and diagnostic output.
-   [X] Test packages without a release for a candidate Python version.
-   [/] Test empty files and projects without dependencies.
-   [/] Test malformed files and unreadable paths.
-   [/] Test duplicate and normalized package names.
-   [X] Test platform markers and unavailable metadata.

### Reference validation

-   [X] Compare selected outcomes with pip or uv where feasible.
-   [/] Create fixtures with known expected outcomes.
-   [X] Document deliberate differences from pip or uv.
-   [/] Prevent regressions with automated tests.

## Phase 8: Diagnostics, CLI, performance, and resilience

### Explanations

-   [X] Identify the package responsible for excluding a Python version.
-   [/] Show the relevant version specifier and marker.
-   [/] Show source file and line number where possible.
-   [X] Explain conflicts between requirements.
-   [X] Distinguish compatible, incompatible, and unknown outcomes.
-   [X] Warn about missing or stale metadata.
-   [X] Summarize why each candidate passed or failed.

### CLI behavior

-   [/] Keep the existing command-line interface predictable.
-   [/] Validate project-directory arguments.
-   [/] Report invalid paths clearly.
-   [/] Define exit codes for success, incompatibility, and analysis
    errors.
-   [X] Keep stdout suitable for piping.
-   [/] Send diagnostics to stderr where appropriate.
-   [X] Consider machine-readable JSON output.
-   [/] Consider verbose/debug output.

### Performance and resilience

-   [X] Avoid fetching identical package metadata repeatedly.
-   [X] Cache repeated marker and version calculations where useful.
-   [/] Handle large requirements files efficiently.
-   [/] Handle interrupted network requests gracefully.
-   [/] Avoid panics on user-controlled input.
-   [/] Keep errors contextual and actionable.

## Phase 9: Rust code quality and release process

### Code structure

-   [/] Separate parsing from evaluation.
-   [/] Separate version parsing from comparison.
-   [X] Separate marker parsing from evaluation.
-   [/] Separate file loading from dependency resolution.
-   [/] Use explicit types for requirements, markers, constraints, and
    results.
-   [/] Keep public APIs stable where practical.
-   [X] Document tricky PEP 440 and PEP 508 behavior.

### Quality gates

-   [/] Run `cargo fmt`.
-   [/] Run `cargo test`.
-   [X] Run `cargo clippy` and address relevant warnings.
-   [/] Add unit tests for every new behavior.
-   [X] Add integration tests using temporary project directories.
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

-   [X] Evaluate operating-system-specific markers.
-   [X] Distinguish CPython from alternative implementations where
    relevant.
-   [X] Consider architecture-specific wheel availability.
-   [X] Consider ABI and platform tags.
-   [/] Distinguish metadata-level compatibility from installability on
    a specific machine.

### Project ecosystems

-   [/] Improve support for common `pyproject.toml` configurations.
-   [X] Improve Poetry, PDM, and other ecosystem-specific parsing.
-   [X] Handle documented lockfile formats.
-   [X] Explain differences between ecosystem resolution rules and
    standard requirements syntax.

### Developer integrations

-   [X] Add JSON output for CI pipelines.
-   [/] Provide stable exit codes for automation.
-   [X] Consider GitHub Actions documentation.
-   [/] Consider a library API alongside the CLI.
-   [X] Consider reports comparing Python versions.
-   [X] Consider analysis of a specific target environment.

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
