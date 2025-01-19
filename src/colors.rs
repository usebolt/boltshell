use std::fmt;

pub struct Color(pub &'static str);

impl Color {
    pub const RED: Color = Color("\x1b[31m");
    pub const GREEN: Color = Color("\x1b[32m");
    pub const YELLOW: Color = Color("\x1b[33m");
    pub const BLUE: Color = Color("\x1b[34m");
    pub const MAGENTA: Color = Color("\x1b[35m");
    pub const CYAN: Color = Color("\x1b[36m");
    pub const RESET: Color = Color("\x1b[0m");
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn colorize(text: &str, color: Color) -> String {
    format!("{}{}{}", color, text, Color::RESET)
} 