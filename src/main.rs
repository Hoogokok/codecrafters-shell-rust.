#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    //<command_name>: command not found for an unrecognized command
    let command = input.trim();
    if command != "echo" {
        eprintln!("{}: command not found", command);
        std::process::exit(1);
    }
}
