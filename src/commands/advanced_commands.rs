use std::fs;
use std::thread;
use std::time::Duration;
use std::process::Command;
use chrono::Local;
use crate::colors::{Color, colorize};

pub fn grep(args: Vec<String>) -> Result<(), String> {
    // TODO: Implement grep functionality
    Ok(())
}

pub fn watch(args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: watch <seconds> <command> [args...]".to_string());
    }

    let seconds = args[0].parse::<u64>()
        .map_err(|_| "Invalid number of seconds")?;
    
    let command = &args[1];
    let command_args = args[2..].to_vec();

    println!("{}", colorize("Press Ctrl+C to stop watching", Color::YELLOW));
    
    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        // Print header
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{}", colorize(&format!("Every {}s: {} {}", 
            seconds, 
            command, 
            command_args.join(" ")), 
            Color::CYAN
        ));
        println!("{}", colorize(&timestamp, Color::GREEN));
        println!();

        // Execute the command
        let output = Command::new(command)
            .args(&command_args)
            .output()
            .map_err(|e| e.to_string())?;

        // Print command output
        if output.status.success() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("{}", colorize(
                &String::from_utf8_lossy(&output.stderr),
                Color::RED
            ));
        }

        thread::sleep(Duration::from_secs(seconds));
    }
} 