use std::{env, process::ExitCode};

use command_line_app::parse_args;

fn main() -> ExitCode {
    match parse_args(env::args().skip(1)) {
        Ok(config) => {
            println!("Preparando {} × {}", config.quantity, config.product);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {error}\nUso: command-line-app PRODUCTO CANTIDAD");
            ExitCode::FAILURE
        }
    }
}
