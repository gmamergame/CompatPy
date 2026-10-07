# CompatPy

CompatPy analyzes a Python project's dependencies, determines which Python versions
they support, and recommends a compatible version.

## Development setup

Install the Rust toolchain, then run the project from the repository root:

```powershell
cargo run -- --help
cargo test
```

The initial Rust CLI provides help and version information. The analysis workflow
will be implemented in these stages:

1. Discover dependencies from project files and Python imports.
2. Retrieve package metadata, including `Requires-Python`, from PyPI.
3. Intersect version constraints and explain incompatible requirements.
4. Recommend a Python version and report the evidence behind it.
5. Add environment creation after analysis and reporting are reliable.
