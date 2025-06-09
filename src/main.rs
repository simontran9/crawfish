use crawfish::cli::{arg_parser, builder, runner};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match arg_parser::parse_args(&args) {
        Ok(command) => match command {
            arg_parser::Command::Build(path) => {
                if let Err(e) = builder::build(&path) {
                    eprintln!("Error: Compilation failure. {:?}", e);
                    process::exit(1);
                }
            }
            arg_parser::Command::Run(path) => {
                if let Err(e) = runner::run(&path) {
                    eprintln!("Error: Run failure. {:?}", e);
                    process::exit(1);
                }
            }
            arg_parser::Command::Help => arg_parser::help(),
            arg_parser::Command::Version => arg_parser::version(),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
