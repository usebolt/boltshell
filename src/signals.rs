use crate::colors::{Color, colorize};

pub fn set_signal_handlers() {
    ctrlc::set_handler(move || {
        println!("\n{}", colorize("Use 'exit' to leave the shell.", Color::YELLOW));
    }).expect("Error setting Ctrl-C handler");
}
