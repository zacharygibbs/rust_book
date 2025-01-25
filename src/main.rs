use std::error::Error;
use std::process;

mod ch2;
mod ch3;
mod ch4;
mod ch5;

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
        Some("2") => {
            ch2::main();
        }
        Some("3") => {
            ch3::main();
        }
        Some("4") => {
            ch4::main();
        }
        Some("5") => {
            ch5::main();
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
    println!("  2     Run the number guessing game");
    println!("  3     variables, types etc.");
    println!("  4     Ownership and Memory");
    println!("  5     Strucs to structure related data");
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
