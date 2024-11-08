use clap::Parser;
use std::process::{Command, Output};

#[derive(Parser)]
struct Cli {
    #[arg()]
    command: String,
}

fn run_command(command: &str) -> Result<Output, String> {
    Command::new(command).
    output()
    .map_err(|err| format!("Failed to Execute Command: {}", err))
}
fn main() {
    let args = Cli::parse();

    match run_command(&args.command) {
        Ok(output) => {
            println!("Output: {}", String::from_utf8_lossy(&output.stdout))
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    // println!("Executing command: {}", String::from_utf8_lossy(&output.stdout));
}
