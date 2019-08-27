const RESET_CODE: &'static str = "\x1B[0m";

/// Simple ANSI predefined codes
pub enum AnsiColorCode {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

pub fn color_8(text: String, color: u8) -> String {
    format!("\x1B[38;5;{}m{}{}", color, text, RESET_CODE)
}

pub fn color_rgb(text: String, red: u8, green: u8, blue: u8) -> String {
    format!("\x1B[38;2;{};{};{}m{}{}", red, green, blue, text, RESET_CODE)
}

pub fn color_code(text: String, color_code: &AnsiColorCode) -> String {
    let color = match color_code {
        AnsiColorCode::Black => "30",
        AnsiColorCode::Red => "31",
        AnsiColorCode::Green => "32",
        AnsiColorCode::Yellow => "33",
        AnsiColorCode::Blue => "34",
        AnsiColorCode::Magenta => "35",
        AnsiColorCode::Cyan => "36",
        AnsiColorCode::White => "37",
        AnsiColorCode::BrightBlack => "90",
        AnsiColorCode::BrightRed => "91",
        AnsiColorCode::BrightGreen => "92",
        AnsiColorCode::BrightYellow => "93",
        AnsiColorCode::BrightBlue => "94",
        AnsiColorCode::BrightMagenta => "95",
        AnsiColorCode::BrightCyan => "96",
        AnsiColorCode::BrightWhite => "97",

    };
    format!("\x1B[{}m{}{}", color, text, RESET_CODE)
}
