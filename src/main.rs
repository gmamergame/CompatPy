use std::env;
use std::path::Path;

use compatpy::discover_dependencies;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: compatpy <project-directory>");
        std::process::exit(1);
    }

    let project_path = Path::new(&args[1]);

    if !project_path.exists() || !project_path.is_dir() {
        eprintln!(
            "Error: '{}' is not a valid directory.",
            project_path.display()
        );
        std::process::exit(1);
    }

    println!("Scanning");
    println!("{}", project_path.display());

    match discover_dependencies(project_path) {
        Ok(dependencies) => {
            if dependencies.is_empty() {
                println!("No dependencies found.");
                return;
            }

            println!();
            println!("Dependencies:");

            for dependency in dependencies {
                println!(
                    "  {} | required: {} | installed: {} | ({})",
                    dependency.name,
                    dependency.specifier.as_deref().unwrap_or("any"),
                    dependency.version.as_deref().unwrap_or("not installed"),
                    dependency.source.display()
                );
            }
        }
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}
