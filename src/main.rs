mod commands;
#[cfg(unix)]
mod signals;
mod colors;
mod updater;

use std::io::{self, Write};
use crate::commands::CommandManager;
#[cfg(unix)]
use crate::signals::set_signal_handlers;
use whoami::{username, fallible};
use std::env;
use crate::colors::{Color, colorize};

const VERSION: &str = "0.1.1";

fn get_prompt() -> String {
    let username = username();
    let hostname = fallible::hostname().unwrap_or_else(|_| String::from("unknown"));
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("?"))
        .to_string_lossy()
        .to_string();
    
    format!("{}", colorize(&format!("{}@{} <{}> ", username, hostname, current_dir), Color::CYAN))
}

fn main() {
    println!("{}", colorize(&format!("Welcome to Bolt v{}", VERSION), Color::GREEN));
    println!("{}\n", colorize("Type 'help' for a list of commands or 'exit' to quit.", Color::YELLOW));

    // Check for updates
    if let Ok(Some(new_version)) = updater::check_for_updates(VERSION) {
        if updater::prompt_update(&new_version) {
            if let Err(e) = updater::install_update() {
                println!("{}", colorize(&format!("Update error: {}", e), Color::RED));
            }
        }
    }

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
                println!("{}", colorize(&format!("Error: {}", e), Color::RED));
            }
        } else {
            if let Err(e) = command_manager.execute_external(command, args) {
                println!("{}", colorize(&format!("Error: {}", e), Color::RED));
            }
        }
    }
}
