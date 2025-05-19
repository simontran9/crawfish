use std::env;
use crawfish::cli::{self, Command};
use crawfish::{builder, runner};

fn main() {
    let args: Vec<String> = env::args().collect();
    match cli::parse_args(&args) {
        Ok(command) => match command {
            Command::Build(path) => {
                if let Err(e) = builder::build(&path) {
                    eprintln!("Error: Compilation failure. {:?}", e); // TODO: remove :?
                    std::process::exit(1);
                }
            }
            Command::Run(path) => {
                if let Err(e) = runner::run(&path) {
                    eprintln!("Error: Run failure. {:?}", e); // TODO: remove :?
                    std::process::exit(1);
                }
            }
            Command::Help => cli::help(),
            Command::Version => cli::version(),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
