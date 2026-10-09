# CompatPy

Find Python versions compatible with your project's dependencies.

CompatPy analyzes a Python project, checks its dependencies against PyPI metadata, and determines which Python versions are compatible with the dependency requirements.

## Features

- Discover dependencies from `requirements.txt`
- Discover dependencies from `pyproject.toml`
- Resolve dependency version constraints
- Retrieve package metadata from PyPI
- Analyze Python version compatibility
- Handle stable and prerelease package versions
- Support Python version exclusions
- Recommend compatible Python versions
- Simple command-line interface

## Installation

### Download

Pre-built binaries are available from the [GitHub Releases](https://github.com/oluxid/CompatPy/releases) page.

For Windows x86-64, download:

```text
compatpy-windows-x86_64.exe
```

Then run it from PowerShell:

```powershell
.\compatpy-windows-x86_64.exe "C:\path\to\your\project"
```

> A native Windows installer is planned for a future release. The installer will make it possible to run `compatpy` directly from PowerShell without specifying the executable path.

### From source

If you have Rust installed:

```powershell
cargo install --git https://github.com/oluxid/CompatPy.git
```

Or clone the repository and build it locally:

```powershell
git clone https://github.com/oluxid/CompatPy.git
cd CompatPy
cargo build --release
```

The compiled executable will be located in:

```text
target/release/
```

## Usage

Pass the root directory of a Python project:

```powershell
compatpy "C:\path\to\your\project"
```

When running a downloaded executable directly on Windows:

```powershell
.\compatpy-windows-x86_64.exe "C:\path\to\your\project"
```

CompatPy will discover the project's dependencies, inspect their package metadata, and determine which Python versions satisfy the dependency requirements.

### Help

```powershell
compatpy --help
```

### Version

```powershell
compatpy --version
```

## Supported project files

| File               | Dependency discovery |
| ------------------ | -------------------- |
| `requirements.txt` | Yes                  |
| `pyproject.toml`   | Yes                  |

## How it works

CompatPy follows a simple process:

```text
Python project
      │
      ▼
Discover dependencies
      │
      ▼
Retrieve package metadata
      │
      ▼
Resolve dependency versions
      │
      ▼
Analyze Python compatibility
      │
      ▼
Recommend compatible Python versions
```

The result is based on the compatibility information available from the project's dependencies and their PyPI metadata.

## Requirements

### Running a release binary

No Rust installation is required when using a pre-built release binary.

An internet connection is required when CompatPy needs to retrieve package metadata from PyPI.

### Building from source

Building CompatPy from source requires:

- Rust 1.85 or newer
- Cargo
- Internet access for retrieving dependencies and PyPI metadata

## Development

Clone the repository:

```powershell
git clone https://github.com/oluxid/CompatPy.git
cd CompatPy
```

Run the test suite:

```powershell
cargo test
```

Build a release binary:

```powershell
cargo build --release
```

Format the code:

```powershell
cargo fmt
```

## Contributing

Want to help improve CompatPy? Pick an item from the roadmap and help move it forward!

Check the current implementation before starting work.
For larger changes, open an issue to discuss the proposed approach first.
Keep changes focused and include tests where appropriate.
Update the roadmap when a feature is implemented and verified.

Not every roadmap item is a commitment or a release blocker. Contributions, bug reports, tests, and suggestions are all welcome.

## License

CompatPy is licensed under the MIT License.

See [`LICENSE`](LICENSE) for the full license text.
