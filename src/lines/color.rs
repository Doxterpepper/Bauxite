const RESET_CODE: &'static str = "\x1B[0m";
const ANSI_ESCAPE: &'static str = "\x1B[";
const ANSI_FORGROUND_COLOR: &'static str = "38;5;";

pub fn color_text(text: String, color: u8) -> String {
    format!("{}{}{}", color_code(color), text, RESET_CODE)
}

pub fn color_code(color: u8) -> String {
    format!("{}{}{}m", ANSI_ESCAPE, ANSI_FORGROUND_COLOR, color.to_string())
}
