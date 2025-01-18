pub fn parse_command(trimmed: &str) {
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let command = parts[0];
    let args = &parts[1..];

    // Use `command` and `args` as needed.
    println!("Command: {}", command);
    println!("Arguments: {:?}", args);
}
