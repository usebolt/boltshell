use std::fs;
use std::process::Command;
use std::env;
use chrono::Local;

pub fn cat(args: Vec<String>) -> Result<(), String> {
    let filename = args.get(0).ok_or("No file specified")?;
    let content = fs::read_to_string(filename)
        .map_err(|e| e.to_string())?;
    println!("{}", content);
    Ok(())
}

pub fn pwd(_args: Vec<String>) -> Result<(), String> {
    let path = env::current_dir()
        .map_err(|e| e.to_string())?;
    println!("{}", path.display());
    Ok(())
}

pub fn date(_args: Vec<String>) -> Result<(), String> {
    let now = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}

pub fn whoami(_args: Vec<String>) -> Result<(), String> {
    println!("{}", whoami::username());
    Ok(())
}

pub fn mkdir(args: Vec<String>) -> Result<(), String> {
    let dir_name = args.get(0).ok_or("No directory name specified")?;
    fs::create_dir_all(dir_name)
        .map_err(|e| e.to_string())
}

pub fn rm(args: Vec<String>) -> Result<(), String> {
    let path = args.get(0).ok_or("No path specified")?;
    if args.contains(&"-r".to_string()) || args.contains(&"-rf".to_string()) {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }.map_err(|e| e.to_string())
} 