mod commands;
#[cfg(unix)]
mod signals;

use std::io::{self, Write};
use crate::commands::CommandManager;
#[cfg(unix)]
use crate::signals::set_signal_handlers;
use whoami::{username, fallible};
use std::env;

const VERSION: &str = "0.1.0";

fn get_prompt() -> String {
    let username = username();
    let hostname = fallible::hostname().unwrap_or_else(|_| String::from("unknown"));
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("?"))
        .to_string_lossy()
        .to_string();
    
    format!("{}@{} <{}> ", username, hostname, current_dir)
}

fn main() {
    println!("Welcome to Bolt v{}", VERSION);
    println!("Type 'help' for a list of commands or 'exit' to quit.\n");

    let command_manager = CommandManager::new();
    #[cfg(unix)]
    set_signal_handlers();

    loop {
        print!("{}", get_prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed == "exit" {
            break;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        if let Some(cmd) = command_manager.get_command(command) {
            if let Err(e) = (cmd.handler)(args) {
                println!("Error: {}", e);
            }
        } else {
            if let Err(e) = command_manager.execute_external(command, args) {
                println!("Error: {}", e);
            }
        }
    }
}
