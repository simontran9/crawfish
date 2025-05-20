use crawfish::cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match cli::arg_parser::parse_args(&args) {
        Ok(command) => match command {
            cli::Command::Build(path) => {
                if let Err(e) = cli::builder::build(&path) {
                    eprintln!("Error: Compilation failure. {:?}", e); // TODO: remove :?
                    std::process::exit(1);
                }
            }
            cli::Command::Run(path) => {
                if let Err(e) = cli::runner::run(&path) {
                    eprintln!("Error: Run failure. {:?}", e); // TODO: remove :?
                    std::process::exit(1);
                }
            }
            cli::Command::Help => cli::help(),
            cli::Command::Version => cli::version(),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
