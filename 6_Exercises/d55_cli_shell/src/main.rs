// Day 55: Project: Build a CLI Shell (Mini Terminal)
// Create a basic interactive shell that accepts commands like ls, cd, pwd, and exit. 
// This project teaches you how to read input in a loop, execute system commands, and 
// simulate a real command-line interface.

// Key Concepts:
// + env::set_current_dir() for cd.
// + Command::new() for executing external programs.
// + Stdio::inherit() to pass through I/O streams.
// You’ve just created your own command interpreter, useful for building embedded 
// shells, CLI tools, or custom scripting environments.
use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    println!("Rust Mini Shell (type 'exit' to quit)");

    loop {
        print!("rust-shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }

        let input = input.trim();
        if input == "exit" {
            println!("Exiting shell!");
            break;
        }

        if input.starts_with("cd ") {
            let path = input.strip_prefix("cd ").unwrap().trim();
            let result = env::set_current_dir(path);
            if let Err(e) = result {
                println!("cd failed: {}", e);
            }
            continue;
        }

        if input == "pwd" {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("pwd failed: {}", e),
            }
            continue;
        }

        run_command(&input);
    }
}

fn run_command(command_line: &str) {
    let parts: Vec<&str> = command_line.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }
 
    let (cmd, args) = parts.split_first().unwrap();
 
    match Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            println!("Command failed: {}", e);
        }
    }
}
