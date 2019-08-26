pub const RESET_CODE: &'static str = "\x1B[0m";

pub fn color_text(text: &String, color: u8) -> String {
    format!("{}{}{}", color_code(color), text, RESET_CODE)
}

pub fn color_code(color: u8) -> String {
    format!("\x1B[38;5;{}m", color.to_string())
}
