const RESET_CODE: &'static str = "\x1B[0m";

pub fn color_8(text: String, color: u8) -> String {
    format!("{}{}{}", ansi_color_code(color), text, RESET_CODE)
}

pub fn color_rgb(text: String, red: u8, green: u8, blue: u8) -> String {
    format!("\x1B[38;2;{};{};{}m{}{}", red, green, blue, text, RESET_CODE)
}

fn ansi_color_code(color: u8) -> String {
    format!("\x1B[38;5;{}m", color.to_string())
}
