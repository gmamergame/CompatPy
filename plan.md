from pathlib import Path

content = """# CompatPy - Project Plan

> **CompatPy** analyzes a Python project, determines which Python versions its dependencies are compatible with, and helps the user create an environment using a compatible Python version.

## 1. Project Goal

Solve a common Python developer problem:

> "I found a Python project, but the author never told me which Python version it was made for."

CompatPy should scan a project, inspect its dependencies and package metadata, calculate the Python versions that are compatible according to the available metadata, recommend the best version, and eventually help set up that environment automatically.

### Core promise

**Scan → Resolve → Recommend → Set up**

CompatPy should make it possible to go from an unknown Python project to a working Python environment with as little manual dependency archaeology as possible.

---

## 2. Stardance Submission Requirements

CompatPy should explicitly satisfy the submission requirements.

### 2.1 Clear quality-of-life improvement

CompatPy removes the need to manually:

- identify every dependency
- look up each package on PyPI
- inspect `Requires-Python`
- compare Python version ranges
- figure out which Python version satisfies all dependencies
- manually install and configure another Python version

### 2.2 Working and usable project

The MVP must be able to:

1. Accept a Python project directory.
2. Detect dependencies.
3. Retrieve package metadata from PyPI.
4. determine supported Python versions.
5. Calculate the compatible Python version range/set.
6. Recommend a Python version.
7. Produce a clear report.

### 2.3 Effort and thoughtful execution

Planned features include:

- multiple dependency sources
- AST-based import detection
- proper version-specifier handling
- clear incompatibility explanations
- existing virtual-environment detection
- Python Install Manager integration
- safe environment creation
- useful error handling
- polished UI

### 2.4 Clear explanation of the problem

The project presentation should demonstrate the problem with a real-world example:

```text
Clone project
    ↓
"What Python version does this need?"
    ↓
Search README
    ↓
Nothing
    ↓
Check requirements.txt
    ↓
Still unclear
    ↓
Look up every package
    ↓
Manually compare Python requirements
```
