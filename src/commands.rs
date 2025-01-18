use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::env;
use std::io;
use std::process;

pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub handler: fn(Vec<String>) -> Result<(), String>,
}

pub struct CommandManager {
    commands: HashMap<String, Command>,
}

pub(crate) mod advanced_commands {
    pub fn grep(_args: Vec<String>) -> Result<(), String> {
        // TODO: Implement grep functionality
        Ok(())
    }

    pub fn watch(_args: Vec<String>) -> Result<(), String> {
        // TODO: Implement watch functionality
        Ok(())
    }
}

impl Command {
    pub fn new(name: &str, description: &str, usage: &str, handler: fn(Vec<String>) -> Result<(), String>) -> Self {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            usage: usage.to_string(),
            handler,
        }
    }
}

impl CommandManager {
    pub fn new() -> Self {
        let mut cm = CommandManager {
            commands: HashMap::new(),
        };
        cm.register_basic_commands();
        cm.register_advanced_commands();
        cm
    }

    pub fn register_command(&mut self, command: Command) {
        self.commands.insert(command.name.clone(), command);
    }

    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }

    fn register_basic_commands(&mut self) {
        self.register_command(Command::new(
            "cd",
            "Change current directory",
            "cd <directory>",
            basic_commands::change_directory
        ));

        self.register_command(Command::new(
            "ls",
            "List files and directories",
            "ls [directory]",
            basic_commands::list_directory
        ));

        self.register_command(Command::new(
            "clear",
            "Clear the terminal screen",
            "clear",
            basic_commands::clear_screen
        ));

        self.register_command(Command::new(
            "help",
            "Show available commands",
            "help [command]",
            basic_commands::show_help
        ));

        self.register_command(Command::new(
            "cat",
            "Display file contents",
            "cat <file>",
            extra_commands::cat
        ));

        self.register_command(Command::new(
            "pwd",
            "Print working directory",
            "pwd",
            extra_commands::pwd
        ));

        self.register_command(Command::new(
            "date",
            "Show current date and time",
            "date",
            extra_commands::date
        ));

        self.register_command(Command::new(
            "whoami",
            "Show current user",
            "whoami",
            extra_commands::whoami
        ));

        self.register_command(Command::new(
            "mkdir",
            "Create new directory",
            "mkdir <directory>",
            extra_commands::mkdir
        ));

        self.register_command(Command::new(
            "rm",
            "Remove file or directory",
            "rm [-r|-rf] <path>",
            extra_commands::rm
        ));
    }

    fn register_advanced_commands(&mut self) {
        self.register_command(Command::new(
            "grep",
            "Search for patterns in files",
            "grep <pattern> <file>",
            advanced_commands::grep
        ));

        self.register_command(Command::new(
            "watch",
            "Execute a command periodically",
            "watch <seconds> <command>",
            advanced_commands::watch
        ));
    }

    pub fn execute_external(&self, command: &str, args: Vec<String>) -> io::Result<()> {
        let mut cmd = process::Command::new(command);
        cmd.args(args);
        let mut child = cmd.spawn()?;
        child.wait()?;
        Ok(())
    }
}

// Implement basic commands in a separate module
mod basic_commands {
    use super::*;

    pub fn change_directory(args: Vec<String>) -> Result<(), String> {
        let new_dir = args.get(0).ok_or("No directory specified")?;
        env::set_current_dir(new_dir).map_err(|e| e.to_string())
    }

    pub fn list_directory(args: Vec<String>) -> Result<(), String> {
        let path = args.get(0).map(Path::new).unwrap_or_else(|| Path::new("."));
        let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
        Ok(())
    }

    pub fn clear_screen(_args: Vec<String>) -> Result<(), String> {
        print!("\x1B[2J\x1B[1;1H");
        Ok(())
    }

    pub fn show_help(args: Vec<String>) -> Result<(), String> {
        let command_manager = CommandManager::new();
        
        if let Some(cmd_name) = args.get(0) {
            if let Some(cmd) = command_manager.get_command(cmd_name) {
                println!("{}: {}", cmd.name, cmd.description);
                println!("Usage: {}", cmd.usage);
            } else {
                println!("Command '{}' not found", cmd_name);
            }
        } else {
            println!("Available commands:");
            for cmd in command_manager.commands.values() {
                println!("{}: {}", cmd.name, cmd.description);
            }
        }
        Ok(())
    }
}

mod extra_commands {
    use std::fs;
    use std::env;

    pub fn cat(args: Vec<String>) -> Result<(), String> {
        let path = args.get(0).ok_or("No file specified")?;
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        println!("{}", content);
        Ok(())
    }

    pub fn pwd(_args: Vec<String>) -> Result<(), String> {
        let path = env::current_dir().map_err(|e| e.to_string())?;
        println!("{}", path.display());
        Ok(())
    }

    pub fn date(_args: Vec<String>) -> Result<(), String> {
        println!("{}", chrono::Local::now());
        Ok(())
    }

    pub fn whoami(_args: Vec<String>) -> Result<(), String> {
        println!("{}", whoami::username());
        Ok(())
    }

    pub fn mkdir(args: Vec<String>) -> Result<(), String> {
        let path = args.get(0).ok_or("No directory specified")?;
        fs::create_dir(path).map_err(|e| e.to_string())
    }

    pub fn rm(args: Vec<String>) -> Result<(), String> {
        let path = args.get(0).ok_or("No path specified")?;
        if args.contains(&"-r".to_string()) || args.contains(&"-rf".to_string()) {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }.map_err(|e| e.to_string())
    }
}
