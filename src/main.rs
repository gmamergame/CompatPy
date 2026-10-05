use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: python-package-compat-tester <project-directory>");
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
}
