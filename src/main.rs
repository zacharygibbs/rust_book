use std::error::Error;
use std::process;

mod ex1;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Config {
    command: Option<String>,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next(); // Skip program name
        let command = args.next();
        Ok(Config { command })
    }
}

fn run(config: Config) -> Result<()> {
    match config.command.as_deref() {
        Some("-h") | Some("--help") => {
            print_help();
        }
        Some("1") => {
            ex1::main();
        }
        None => {
            print_help();
        }
        Some(cmd) => {
            return Err(format!("Unknown command: {}", cmd).into());
        }
    }
    Ok(())
}

fn print_help() {
    println!("Usage: rust_book <command>");
    println!("Commands:");
    println!("  1     Run the number guessing game");
    println!("  -h    Show this help message");
}

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
