#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        //<command_name>: command not found for an unrecognized command
        let command = input.trim();
        let command_parts: Vec<&str> = deconstruct_command(command);
        match command_parts[0] {
            "exit 0" => {
                return;
            }
            "echo" => {
                println!("{}", command_parts[1..].join(" "));   
            }
            _ => {
                eprintln!("{}: command not found", command);
            }
        }
    }
}
// exit 0: exit the shell
// echo <text>: print <text> to the console
fn deconstruct_command(command: &str) -> Vec<&str> {
    if command.trim() == "exit 0" {
        vec![command.trim()]
    } else {
        command.split_whitespace().collect()
    }
}
