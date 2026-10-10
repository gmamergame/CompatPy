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
