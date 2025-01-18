use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CustomCommand {
    pub name: String,
    pub description: String,
    pub script: String,
    pub args: Vec<String>,
}

pub fn load_custom_commands(path: &str) -> Result<Vec<CustomCommand>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;
    
    serde_json::from_str(&content)
        .map_err(|e| e.to_string())
} 