use std::process::Command;
use std::io::Write;
use crate::colors::{Color, colorize};
use serde::Deserialize;
use reqwest::blocking::Client;

const CRATE_NAME: &str = "boltshell";

#[derive(Deserialize)]
struct CrateVersion {
    version: String,
}

#[derive(Deserialize)]
struct CrateResponse {
    versions: Vec<CrateVersion>,
}

pub fn check_for_updates(current_version: &str) -> Result<Option<String>, String> {
    println!("{}", colorize("Checking for updates...", Color::BLUE));
    
    let client = Client::new();
    let url = format!("https://crates.io/api/v1/crates/{}", CRATE_NAME);
    
    let response = client
        .get(&url)
        .header("User-Agent", "boltshell")
        .send()
        .map_err(|e| e.to_string())?;
        
    let crate_info: CrateResponse = response
        .json()
        .map_err(|e| e.to_string())?;
        
    if let Some(latest) = crate_info.versions.first() {
        if latest.version != current_version {
            return Ok(Some(latest.version.clone()));
        }
    }
    
    Ok(None)
}

pub fn prompt_update(new_version: &str) -> bool {
    print!("{} [y/N] ", 
        colorize(
            &format!("Version {} is available. Would you like to update?", new_version),
            Color::BLUE
        )
    );
    std::io::stdout().flush().unwrap();
    
    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();
    
    response.trim().to_lowercase() == "y"
}

pub fn install_update() -> Result<(), String> {
    println!("{}", colorize("Starting update...", Color::MAGENTA));
    
    // Run cargo install with --force to update
    let status = Command::new("cargo")
        .args(["install", "--force", CRATE_NAME])
        .status()
        .map_err(|e| e.to_string())?;
        
    if status.success() {
        println!("{}", colorize("Update completed successfully! Please restart boltshell.", Color::GREEN));
        Ok(())
    } else {
        Err("Update failed. Please try again or install manually.".to_string())
    }
} 