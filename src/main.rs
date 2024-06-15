use std::env;
use std::fs;
use std::io::{self, Write};
use std::ops::ControlFlow;
use std::path::Path;
use std::process::Command;
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
            identify_command(command_parts[1..].to_vec());
        }
        "pwd" => {
            let current_dir = env::current_dir().unwrap();
            println!("{}", current_dir.display());
        }
        "cd" => change_directory(&command_parts),
        _ => {
            //외부 명령어 실행
            let output = Command::new(command_parts[0])
                .args(&command_parts[1..])
                .output();

            match output {
                Ok(output) => {
                    if !output.stdout.is_empty() {
                        println!("{}", String::from_utf8_lossy(&output.stdout).trim());
                    }
                    if !output.stderr.is_empty() {
                        eprintln!("{}: command not found", command);
                    }
                }
                Err(_e) => {
                    eprintln!("{}: command not found", command);
                }
            }
        }
    }
    ControlFlow::Continue(())
}

fn change_directory(command_parts: &Vec<&str>) {
    let new_dir = command_parts[1];
    let path = Path::new(new_dir);
    match (path.is_dir(), new_dir) {
        (true, _) => env::set_current_dir(&path).unwrap(),
        (_, "~") => {
            let home_dir = env::var("HOME").unwrap();
            env::set_current_dir(&home_dir).unwrap();
        },
        _ => eprintln!("cd: {}: No such file or directory", new_dir.trim()),
    }
}

fn identify_command(command_parts: Vec<&str>) {
    let command = command_parts[0];
    match command {
        "exit" => println!("exit is a shell builtin"),
        "echo" => println!("echo is a shell builtin"),
        "type" => println!("type is a shell builtin"),
        "cat" => println!("cat is /bin/cat"),
        _ => {
            let path = env::var("PATH").unwrap();
            let paths: Vec<&str> = path.split(':').collect();
            let mut found = false;
            for p in paths {
                let full_path = Path::new(p).join(command);
                if fs::metadata(full_path.clone()).is_ok() {
                    println!("{} is {}", command, full_path.display());
                    found = true;
                    break;
                }
            }
            if !found {
                println!("{}: not found", command);
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
