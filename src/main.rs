use pysure::{Command, VERSION, parse_args};
use std::env;
use std::process;

const HELP: &str = "PySure - find Python versions compatible with a project's dependencies.\n\nUsage: pysure [--help | --version]\n";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match parse_args(&args) {
        Ok(Command::Help) => print!("{HELP}"),
        Ok(Command::Version) => println!("pysure {VERSION}"),
        Err(message) => {
            eprintln!("pysure: {message}");
            process::exit(2);
        }
    }
}
