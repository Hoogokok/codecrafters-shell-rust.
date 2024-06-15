#[allow(unused_imports)]
use std::io::{self, Write};
use std::ops::ControlFlow;

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
        let fun_name = process_command(command_parts, command);
        if let ControlFlow::Break(_) = fun_name {
            return;
        }
    }
}

fn process_command(command_parts: Vec<&str>, command: &str) -> ControlFlow<()> {
    match command_parts[0] {
        "exit 0" => {
            return ControlFlow::Break(());
        }
        "echo" => {
            println!("{}", command_parts[1..].join(" "));   
        }
        "type" => {
            identify_command(command_parts);
        }
        _ => {
            eprintln!("{}: command not found", command);
        }
    }
    ControlFlow::Continue(())
}

fn identify_command(command_parts: Vec<&str>) {
    match command_parts[1] {
        "exit" => {
            println!("exit is a shell builtin");
        }
        "echo" => {
            println!("echo is a shell builtin");
        }
        "type" => {
            println!("type is a shell builtin");
        }
        "cat" => {
            println!("cat is /bin/cat");
        }
        _ => {
            println!("{}: not found", command_parts[1]);
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
