use std::env;
use std::io::{self, Write};
use std::process::Command;

pub fn execute_command(mut command: Command) -> Result<(), io::Error> {
    let mut child = command.spawn()?;
    let status = child.wait()?;
    if !status.success() {
        eprintln!("Command exited with status: {}", status);
    }
    Ok(())
}
