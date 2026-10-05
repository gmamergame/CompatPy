use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args(args: &[String]) -> Result<Command, String> {
    match args {
        [] => Ok(Command::Help),
        [flag] if flag == "--help" || flag == "-h" => Ok(Command::Help),
        [flag] if flag == "--version" || flag == "-v" => Ok(Command::Version),
        _ => Err("unexpected arguments; use --help for usage".to_owned()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependency {
    pub name: String,
    pub source: PathBuf,
}

pub fn discover_dependencies(project_path: &Path) -> Result<Vec<Dependency>, String> {
    let mut dependencies = BTreeSet::new();

    for path in dependency_files(project_path)? {
        let contents = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;

        if path.file_name().and_then(|name| name.to_str()) == Some("pyproject.toml") {
            discover_from_pyproject(&contents, &path, &mut dependencies)?;
        } else {
            discover_from_requirements(&contents, &path, &mut dependencies);
        }
    }

    Ok(dependencies.into_iter().collect())
}

fn dependency_files(project_path: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();

    let requirements = project_path.join("requirements.txt");
    if requirements.is_file() {
        files.push(requirements);
    }

    let requirements_dir = project_path.join("requirements");
    if requirements_dir.is_dir() {
        let entries = fs::read_dir(&requirements_dir)
            .map_err(|error| format!("failed to read '{}': {error}", requirements_dir.display()))?;

        for entry in entries {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to inspect '{}': {error}",
                    requirements_dir.display()
                )
            })?;

            let path = entry.path();

            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
                files.push(path);
            }
        }
    }

    let pyproject = project_path.join("pyproject.toml");
    if pyproject.is_file() {
        files.push(pyproject);
    }

    files.sort();
    Ok(files)
}

fn discover_from_requirements(
    contents: &str,
    source: &Path,
    dependencies: &mut BTreeSet<Dependency>,
) {
    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
            continue;
        }

        let package = line
            .split_once(';')
            .map_or(line, |(requirement, _)| requirement)
            .trim();

        let package = package
            .split(|character: char| {
                character == '<'
                    || character == '>'
                    || character == '='
                    || character == '!'
                    || character == '~'
                    || character == '^'
                    || character == '['
            })
            .next()
            .unwrap_or("")
            .trim();

        if !package.is_empty() {
            dependencies.insert(Dependency {
                name: normalize_package_name(package),
                source: source.to_path_buf(),
            });
        }
    }
}

fn discover_from_pyproject(
    contents: &str,
    source: &Path,
    dependencies: &mut BTreeSet<Dependency>,
) -> Result<(), String> {
    let document: toml::Value = contents
        .parse()
        .map_err(|error| format!("failed to parse '{}': {error}", source.display()))?;

    if let Some(array) = document
        .get("project")
        .and_then(|project| project.get("dependencies"))
        .and_then(toml::Value::as_array)
    {
        for dependency in array.iter().filter_map(toml::Value::as_str) {
            add_requirement(dependency, source, dependencies);
        }
    }

    Ok(())
}

fn add_requirement(requirement: &str, source: &Path, dependencies: &mut BTreeSet<Dependency>) {
    let requirement = requirement
        .split_once(';')
        .map_or(requirement, |(requirement, _)| requirement)
        .trim();

    let package = requirement
        .split(|character: char| {
            character == '<'
                || character == '>'
                || character == '='
                || character == '!'
                || character == '~'
                || character == '^'
                || character == '['
        })
        .next()
        .unwrap_or("")
        .trim();

    if !package.is_empty() {
        dependencies.insert(Dependency {
            name: normalize_package_name(package),
            source: source.to_path_buf(),
        });
    }
}

fn normalize_package_name(name: &str) -> String {
    name.trim()
        .to_ascii_lowercase()
        .replace('_', "-")
        .replace('.', "-")
}

#[cfg(test)]
mod tests {
    use super::{Command, Dependency, discover_dependencies, normalize_package_name, parse_args};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn help_is_the_default_command() {
        assert_eq!(parse_args(&[]), Ok(Command::Help));
    }

    #[test]
    fn parses_version_flag() {
        assert_eq!(parse_args(&["--version".to_owned()]), Ok(Command::Version));
    }

    #[test]
    fn rejects_unexpected_arguments() {
        assert!(parse_args(&["unknown".to_owned()]).is_err());
    }

    #[test]
    fn normalizes_package_names() {
        assert_eq!(normalize_package_name("My_Package.Name"), "my-package-name");
    }

    #[test]
    fn discovers_requirements_txt() {
        let project = temporary_project();
        fs::write(
            project.join("requirements.txt"),
            "# comment\nRequests>=2.0\nFlask==3.0\nrequests\n",
        )
        .unwrap();

        let dependencies = discover_dependencies(&project).unwrap();

        assert_eq!(
            dependencies,
            vec![
                Dependency {
                    name: "flask".to_owned(),
                    source: project.join("requirements.txt"),
                },
                Dependency {
                    name: "requests".to_owned(),
                    source: project.join("requirements.txt"),
                },
            ]
        );

        fs::remove_dir_all(project).unwrap();
    }

    #[test]
    fn discovers_pyproject_dependencies() {
        let project = temporary_project();

        fs::write(
            project.join("pyproject.toml"),
            r#"
[project]
dependencies = [
    "requests>=2.0",
    "Flask",
]
"#,
        )
        .unwrap();

        let dependencies = discover_dependencies(&project).unwrap();

        assert_eq!(dependencies.len(), 2);
        assert_eq!(dependencies[0].name, "flask");
        assert_eq!(dependencies[1].name, "requests");

        fs::remove_dir_all(project).unwrap();
    }

    fn temporary_project() -> std::path::PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("python-package-compat-tester-{timestamp}"));

        fs::create_dir_all(&path).unwrap();
        path
    }
}
