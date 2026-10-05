# PySure

PySure analyzes a Python project's dependencies, determines which Python versions
they support, and recommends a compatible version.

## Development setup

Create and activate a virtual environment, then install the project with its
development tools:

```powershell
py -m venv .venv
.venv\Scripts\Activate.ps1
python -m pip install --upgrade pip
python -m pip install -e ".[dev]"
```

Run the test suite and lint checks:

```powershell
python -m pytest
ruff check .
```

The `pysure` command is the package's CLI entry point. The initial workspace is
set up for implementing the analysis workflow:

1. Discover dependencies from project files and Python imports.
2. Retrieve package metadata, including `Requires-Python`, from PyPI.
3. Intersect version constraints and explain incompatible requirements.
4. Recommend a Python version and report the evidence behind it.
5. Add environment creation after analysis and reporting are reliable.
