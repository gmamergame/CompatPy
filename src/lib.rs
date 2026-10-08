use std::fs;
use std::path::{Path, PathBuf};

macro_rules! test_debug {
    ($($argument:tt)*) => {{
        #[cfg(test)]
        {
            eprintln!("[compat-debug] {}", format_args!($($argument)*));
        }
    }};
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args(args: &[String]) -> Result<Command, String> {
    test_debug!("parse_args: received {args:?}");

    let result = match args {
        [] => Ok(Command::Help),
        [flag] if flag == "--help" || flag == "-h" => Ok(Command::Help),
        [flag] if flag == "--version" || flag == "-v" => Ok(Command::Version),
        _ => Err("unexpected arguments; use --help for usage".to_owned()),
    };

    test_debug!("parse_args: result {result:?}");
    result
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependency {
    pub name: String,
    pub specifier: Option<String>,
    pub marker: Option<String>,
    pub source: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageMetadata {
    pub name: String,
    pub latest_version: String,
    pub requires_python: Option<String>,
    pub releases: Vec<PackageRelease>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRelease {
    pub version: String,
    pub requires_python: Option<String>,
}

pub fn discover_dependencies(project_path: &Path) -> Result<Vec<Dependency>, String> {
    test_debug!(
        "discover_dependencies: project path '{}'",
        project_path.display()
    );
    let mut dependencies = Vec::new();

    let files = dependency_files(project_path)?;
    test_debug!(
        "discover_dependencies: found {} candidate file(s): {files:?}",
        files.len()
    );

    for path in files {
        test_debug!("discover_dependencies: reading '{}'", path.display());
        let contents = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;
        test_debug!(
            "discover_dependencies: contents of '{}':\n{}",
            path.display(),
            contents
        );

        if path.file_name().and_then(|name| name.to_str()) == Some("pyproject.toml") {
            test_debug!(
                "discover_dependencies: parsing '{}' as pyproject.toml",
                path.display()
            );
            discover_from_pyproject(&contents, &path, &mut dependencies)?;
        } else {
            test_debug!(
                "discover_dependencies: parsing '{}' as requirements",
                path.display()
            );
            discover_from_requirements(&contents, &path, &mut dependencies);
        }
    }

    dependencies.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| right.specifier.cmp(&left.specifier))
            .then_with(|| left.marker.cmp(&right.marker))
            .then_with(|| left.source.cmp(&right.source))
    });
    test_debug!("discover_dependencies: final dependencies: {dependencies:#?}");
    Ok(dependencies)
}

pub fn fetch_pypi_metadata(package_name: &str) -> Result<PackageMetadata, String> {
    let normalized_name = normalize_package_name(package_name);

    let url = format!("https://pypi.org/pypi/{normalized_name}/json");

    test_debug!("fetch_pypi_metadata: requesting '{url}'");

    let response = reqwest::blocking::get(&url)
        .map_err(|error| format!("failed to query PyPI for '{normalized_name}': {error}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "PyPI returned HTTP {} for package '{normalized_name}'",
            response.status()
        ));
    }

    let body = response
        .text()
        .map_err(|error| format!("failed to read PyPI response: {error}"))?;

    parse_pypi_metadata(&body)
}

#[derive(Debug, serde::Deserialize)]
struct PyPiResponse {
    info: PyPiInfo,
    releases: std::collections::HashMap<String, Vec<PyPiReleaseFile>>,
}

#[derive(Debug, serde::Deserialize)]
struct PyPiInfo {
    name: String,
    version: String,
    requires_python: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct PyPiReleaseFile {
    requires_python: Option<String>,
}

fn dependency_files(project_path: &Path) -> Result<Vec<PathBuf>, String> {
    test_debug!("dependency_files: scanning '{}'", project_path.display());
    let mut files = Vec::new();

    let requirements = project_path.join("requirements.txt");
    if requirements.is_file() {
        test_debug!("dependency_files: found '{}'", requirements.display());
        files.push(requirements);
    } else {
        test_debug!("dependency_files: '{}' is absent", requirements.display());
    }

    let requirements_dir = project_path.join("requirements");
    if requirements_dir.is_dir() {
        test_debug!(
            "dependency_files: scanning directory '{}'",
            requirements_dir.display()
        );
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
                test_debug!(
                    "dependency_files: found requirements file '{}'",
                    path.display()
                );
                files.push(path);
            } else {
                test_debug!("dependency_files: ignoring entry '{}'", path.display());
            }
        }
    } else {
        test_debug!(
            "dependency_files: '{}' is absent",
            requirements_dir.display()
        );
    }

    let pyproject = project_path.join("pyproject.toml");
    if pyproject.is_file() {
        test_debug!("dependency_files: found '{}'", pyproject.display());
        files.push(pyproject);
    } else {
        test_debug!("dependency_files: '{}' is absent", pyproject.display());
    }

    files.sort();
    test_debug!("dependency_files: sorted candidate files: {files:?}");
    Ok(files)
}

fn discover_from_requirements(contents: &str, source: &Path, dependencies: &mut Vec<Dependency>) {
    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
            test_debug!("requirements: skipping line {line:?}");
            continue;
        }

        test_debug!("requirements: parsing line {line:?}");
        add_requirement(line, source, dependencies);
    }
}

fn discover_from_pyproject(
    contents: &str,
    source: &Path,
    dependencies: &mut Vec<Dependency>,
) -> Result<(), String> {
    let document: toml::Value = toml::from_str(contents)
        .map_err(|error| format!("failed to parse '{}': {error}", source.display()))?;
    test_debug!("pyproject: parsed document: {document:#?}");

    if let Some(array) = document
        .get("project")
        .and_then(|project| project.get("dependencies"))
        .and_then(toml::Value::as_array)
    {
        test_debug!(
            "pyproject: found {} project dependency item(s)",
            array.len()
        );
        for dependency in array.iter().filter_map(toml::Value::as_str) {
            test_debug!("pyproject: parsing dependency {dependency:?}");
            add_requirement(dependency, source, dependencies);
        }
    } else {
        test_debug!("pyproject: no project.dependencies array found");
    }

    Ok(())
}

fn add_requirement(requirement: &str, source: &Path, dependencies: &mut Vec<Dependency>) {
    test_debug!(
        "add_requirement: input {requirement:?} from '{}'",
        source.display()
    );
    let (requirement, marker) = requirement
        .split_once(';')
        .map_or((requirement, None), |(requirement, marker)| {
            (requirement, Some(marker.trim().to_owned()))
        });

    let requirement = requirement.trim();

    let name_end = requirement
        .find(|character: char| {
            character == '['
                || character == '<'
                || character == '>'
                || character == '='
                || character == '!'
                || character == '~'
                || character == '^'
                || character.is_whitespace()
        })
        .unwrap_or(requirement.len());

    let name = requirement[..name_end].trim();

    if name.is_empty() {
        test_debug!("add_requirement: ignoring empty package name");
        return;
    }

    let mut specifier_part = requirement[name_end..].trim();

    // Skip extras such as `foo[bar,baz]`.
    if specifier_part.starts_with('[') {
        if let Some(end) = specifier_part.find(']') {
            specifier_part = specifier_part[end + 1..].trim();
        }
    }

    let specifier = (!specifier_part.is_empty()).then(|| specifier_part.to_owned());

    let normalized_name = normalize_package_name(name);

    let dependency = Dependency {
        name: normalized_name,
        specifier,
        marker,
        source: source.to_path_buf(),
    };

    test_debug!("add_requirement: normalized dependency {dependency:#?}");
    if !dependencies.contains(&dependency) {
        dependencies.push(dependency);
    }
    test_debug!("add_requirement: dependency count={}", dependencies.len());
}

fn normalize_package_name(name: &str) -> String {
    let normalized = name
        .trim()
        .to_ascii_lowercase()
        .replace('_', "-")
        .replace('.', "-");
    test_debug!("normalize_package_name: {name:?} -> {normalized:?}");
    normalized
}

fn parse_pypi_metadata(contents: &str) -> Result<PackageMetadata, String> {
    let metadata: PyPiResponse = serde_json::from_str(contents)
        .map_err(|error| format!("failed to parse PyPI metadata: {error}"))?;

    let mut releases = metadata
        .releases
        .into_iter()
        .map(|(version, files)| {
            let requires_python = files
                .into_iter()
                .filter_map(|file| file.requires_python)
                .next();

            PackageRelease {
                version,
                requires_python,
            }
        })
        .collect::<Vec<_>>();

    releases.sort_by(|left, right| left.version.cmp(&right.version));

    Ok(PackageMetadata {
        name: normalize_package_name(&metadata.info.name),
        latest_version: metadata.info.version,
        requires_python: metadata.info.requires_python,
        releases,
    })
}

pub fn select_package_release<'a>(
    metadata: &'a PackageMetadata,
    specifier: Option<&str>,
) -> Option<&'a PackageRelease> {
    let matching = metadata.releases.iter().filter(|release| {
        specifier
            .map(|specifier| package_version_satisfies(&release.version, specifier))
            .unwrap_or(true)
    });

    let stable = matching
        .clone()
        .filter(|release| !is_prerelease(&release.version))
        .max_by(|left, right| compare_versions(&left.version, &right.version));

    stable
        .or_else(|| matching.max_by(|left, right| compare_versions(&left.version, &right.version)))
}

fn is_prerelease(version: &str) -> bool {
    let parsed = parse_package_version(version);
    parsed.prerelease.is_some()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ParsedPackageVersion {
    release: Vec<u64>,
    prerelease: Option<Prerelease>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Prerelease {
    Alpha(u64),
    Beta(u64),
    Rc(u64),
}

fn compare_versions(left: &str, right: &str) -> std::cmp::Ordering {
    let left_version = parse_package_version(left);
    let right_version = parse_package_version(right);

    match left_version.release.cmp(&right_version.release) {
        std::cmp::Ordering::Equal => match (&left_version.prerelease, &right_version.prerelease) {
            (None, None) => std::cmp::Ordering::Equal,

            //stable releases are newer than prereleases.
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(_), None) => std::cmp::Ordering::Less,

            //both are  prereleases, so compare their actual prerelease versions.
            (Some(left), Some(right)) => left.cmp(right),
        },

        ordering => ordering,
    }
}

fn parse_package_version(version: &str) -> ParsedPackageVersion {
    let version = version.trim().to_ascii_lowercase();

    let (release_part, prerelease) = if let Some(index) = version.find("rc") {
        (
            &version[..index],
            Some(Prerelease::Rc(
                version[index + 2..].parse::<u64>().unwrap_or(0),
            )),
        )
    } else if let Some(index) = version.find('a') {
        (
            &version[..index],
            Some(Prerelease::Alpha(
                version[index + 1..].parse::<u64>().unwrap_or(0),
            )),
        )
    } else if let Some(index) = version.find('b') {
        (
            &version[..index],
            Some(Prerelease::Beta(
                version[index + 1..].parse::<u64>().unwrap_or(0),
            )),
        )
    } else {
        (version.as_str(), None)
    };

    let mut release = release_part
        .split('.')
        .map(|part| part.parse::<u64>().unwrap_or(0))
        .collect::<Vec<_>>();

    while release.len() < 3 {
        release.push(0);
    }

    ParsedPackageVersion {
        release,
        prerelease,
    }
}

fn package_version_satisfies(version: &str, requirement: &str) -> bool {
    requirement
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .all(|part| satisfies_package_specifier(version, part))
}

fn satisfies_package_specifier(version: &str, specifier: &str) -> bool {
    let operators = [">=", "<=", "==", "!=", ">", "<"];

    let (operator, required) = operators
        .iter()
        .find_map(|operator| {
            specifier
                .strip_prefix(operator)
                .map(|required| (*operator, required.trim()))
        })
        .unwrap_or(("==", specifier.trim()));

    let comparison = compare_versions(version, required);

    match operator {
        ">=" => comparison != std::cmp::Ordering::Less,
        "<=" => comparison != std::cmp::Ordering::Greater,
        "==" => comparison == std::cmp::Ordering::Equal,
        "!=" => comparison != std::cmp::Ordering::Equal,
        ">" => comparison == std::cmp::Ordering::Greater,
        "<" => comparison == std::cmp::Ordering::Less,
        _ => false,
    }
}

#[cfg(test)]
mod tests {

    use super::{
        Command, Dependency, discover_dependencies, normalize_package_name, parse_args,
        select_package_release,
    };
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
                    specifier: Some("==3.0".to_owned()),
                    marker: None,
                    source: project.join("requirements.txt"),
                },
                Dependency {
                    name: "requests".to_owned(),
                    specifier: Some(">=2.0".to_owned()),
                    marker: None,
                    source: project.join("requirements.txt"),
                },
                Dependency {
                    name: "requests".to_owned(),
                    specifier: None,
                    marker: None,
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
            r#"[project]
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
        assert_eq!(dependencies[0].specifier, None);
        assert_eq!(dependencies[0].marker, None);

        assert_eq!(dependencies[1].name, "requests");
        assert_eq!(dependencies[1].specifier, Some(">=2.0".to_owned()));
        assert_eq!(dependencies[1].marker, None);

        fs::remove_dir_all(project).unwrap();
    }

    #[test]
    fn parses_pypi_metadata() {
        let json = r#"
        {
            "info": {
                "name": "Example-Package",
                "version": "2.0.0",
                "requires_python": ">=3.9"
            },
            "releases": {
                "1.0.0": [
                    {
                        "requires_python": ">=3.7"
                    }
                ],
                "2.0.0": [
                    {
                        "requires_python": ">=3.9"
                    }
                ]
            }
        }
        "#;

        let metadata = super::parse_pypi_metadata(json).unwrap();

        assert_eq!(metadata.name, "example-package");
        assert_eq!(metadata.latest_version, "2.0.0");
        assert_eq!(metadata.requires_python, Some(">=3.9".to_owned()));

        assert_eq!(
            metadata.releases,
            vec![
                super::PackageRelease {
                    version: "1.0.0".to_owned(),
                    requires_python: Some(">=3.7".to_owned()),
                },
                super::PackageRelease {
                    version: "2.0.0".to_owned(),
                    requires_python: Some(">=3.9".to_owned()),
                },
            ]
        )
    }

    #[test]
    fn selects_exact_package_version() {
        let metadata = super::PackageMetadata {
            name: "uvicorn".to_owned(),
            latest_version: "0.54.0".to_owned(),
            requires_python: Some(">=3.10".to_owned()),
            releases: vec![
                super::PackageRelease {
                    version: "0.29.0".to_owned(),
                    requires_python: Some(">=3.8".to_owned()),
                },
                super::PackageRelease {
                    version: "0.54.0".to_owned(),
                    requires_python: Some(">=3.10".to_owned()),
                },
            ],
        };

        let release = select_package_release(&metadata, Some("==0.29.0")).unwrap();

        assert_eq!(release.version, "0.29.0");
        assert_eq!(release.requires_python, Some(">=3.8".to_owned()));
    }

    #[test]
    fn selects_latest_matching_package_version() {
        let metadata = super::PackageMetadata {
            name: "example".to_owned(),
            latest_version: "2.0.0".to_owned(),
            requires_python: None,
            releases: vec![
                super::PackageRelease {
                    version: "1.0.0".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "1.5.0".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "2.0.0".to_owned(),
                    requires_python: None,
                },
            ],
        };

        let release = select_package_release(&metadata, Some(">=1.0,<2.0")).unwrap();

        assert_eq!(release.version, "1.5.0");
    }

    #[test]
    fn selects_latest_release_without_specifier() {
        let metadata = super::PackageMetadata {
            name: "example".to_owned(),
            latest_version: "2.0.0".to_owned(),
            requires_python: None,
            releases: vec![
                super::PackageRelease {
                    version: "1.0.0".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "2.0.0".to_owned(),
                    requires_python: None,
                },
            ],
        };

        let release = select_package_release(&metadata, None).unwrap();

        assert_eq!(release.version, "2.0.0");
    }

    #[test]
    fn exact_version_does_not_match_prerelease() {
        let metadata = super::PackageMetadata {
            name: "example".to_owned(),
            latest_version: "3.9.0".to_owned(),
            requires_python: None,
            releases: vec![
                super::PackageRelease {
                    version: "3.9.0rc2".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "3.9.0".to_owned(),
                    requires_python: None,
                },
            ],
        };

        let release = select_package_release(&metadata, Some("==3.9.0")).unwrap();

        assert_eq!(release.version, "3.9.0");
    }

    #[test]
    fn stable_release_beats_prerelease() {
        let metadata = super::PackageMetadata {
            name: "example".to_owned(),
            latest_version: "4.0.0".to_owned(),
            requires_python: None,
            releases: vec![
                super::PackageRelease {
                    version: "3.9.0".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "3.9.0rc2".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "4.0.0a1".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "4.0.0".to_owned(),
                    requires_python: None,
                },
            ],
        };

        let release = select_package_release(&metadata, Some(">=3.8.0")).unwrap();

        assert_eq!(release.version, "4.0.0");
    }

    #[test]
    fn prerelease_versions_are_ordered_correctly() {
        assert_eq!(
            super::compare_versions("3.9.0a1", "3.9.0b1"),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            super::compare_versions("3.9.0b1", "3.9.0rc1"),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            super::compare_versions("3.9.0rc1", "3.9.0"),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn prefers_stable_release_over_prerelease() {
        let metadata = super::PackageMetadata {
            name: "example".to_owned(),
            latest_version: "3.1.0rc0".to_owned(),
            requires_python: None,
            releases: vec![
                super::PackageRelease {
                    version: "2.9.0".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "3.0.0".to_owned(),
                    requires_python: None,
                },
                super::PackageRelease {
                    version: "3.1.0rc0".to_owned(),
                    requires_python: None,
                },
            ],
        };

        let release = select_package_release(&metadata, Some(">=2.0.0")).unwrap();

        assert_eq!(release.version, "3.0.0");
    }

    fn temporary_project() -> std::path::PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("compatpy-{timestamp}"));

        fs::create_dir_all(&path).unwrap();
        path
    }
}
