use std::{
    fmt,
    path::{Path, PathBuf},
};

pub enum Command {
    Build(PathBuf),
    Run(PathBuf),
    Help,
    Version,
}

pub enum CLIError {
    FileNotFound(String),
    InvalidFileExtension,
    InvalidCommand(String),
    MissingArgument,
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CLIError::FileNotFound(path) => write!(f, "File not found ({})", path),
            CLIError::InvalidFileExtension => write!(f, "Invalid file extension (expected .crw)"),
            CLIError::InvalidCommand(cmd) => write!(f, "Invalid command ({})", cmd),
            CLIError::MissingArgument => {
                write!(f, "Missing arguments. Use -h or --help for more information.")
            }
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<Command, CLIError> {
    let command = args.get(1).map(String::as_str);
    match (args.len(), command) {
        (3, Some("build" | "run")) => {
            let source_path = PathBuf::from(&args[2]);

            if !source_path.exists() || !source_path.is_file() {
                return Err(CLIError::FileNotFound(args[2].clone()));
            }

            if !has_crw_extension(&source_path) {
                return Err(CLIError::InvalidFileExtension);
            }

            Ok(match command.unwrap() {
                "build" => Command::Build(source_path),
                _ => Command::Run(source_path),
            })
        }
        (2, Some("-h" | "--help")) => Ok(Command::Help),
        (2, Some("-v" | "--version")) => Ok(Command::Version),
        (_, Some(command)) => Err(CLIError::InvalidCommand(command.to_string())),
        _ => Err(CLIError::MissingArgument),
    }
}

pub fn help() {
    let message = r#"Crawfish compiler

Usage: crawfish [OPTIONS] [ARGUMENT]

Options:
    build [file].crw              compile the current file
    run [file].crw                run the current file
    -h, --help                    print possible commands
    -v, --version                 print compiler version"#;
    println!("{}", message);
}

pub fn version() {
    let message = "crawfish 0.0.1";
    println!("{}", message);
}

fn has_crw_extension(p: &Path) -> bool {
    p.extension().and_then(|ext| ext.to_str()) == Some("crw")
}
