pub fn set_signal_handlers() {
    ctrlc::set_handler(move || {
        println!("\nUse 'exit' to leave the shell.");
    }).expect("Error setting Ctrl-C handler");
}
