pub mod ansi_color_codes;
pub mod rgb_color;

use ansi_color_codes::AnsiColorCode;
use rgb_color::RgbColor;

const RESET_CODE: &'static str = "\x1B[0m";

/// Stores the line color information for the box.
/// Only one line color type should have a value at a time.
pub struct LineColor {
    pub ansi: Option<AnsiColorCode>,
    pub rgb: Option<RgbColor>,
    pub color8: Option<u8>,
}

impl LineColor {
    /// Constructs a line color struct with no values for each color.
    pub fn new() -> LineColor {
        LineColor {
            ansi: None,
            rgb: None,
            color8: None,
        }
    }

    /// Wraps the given text in the color specified by the LineColor struct.
    pub fn wrap_color(&self, text: String) -> String {
        if let Some(ansi) = &self.ansi {
            self.color_code(text, &ansi)
        } else if let Some(rgb) = &self.rgb {
            self.color_rgb(text, rgb)
        }
        else if let Some(color8) = &self.color8 {
            self.color_8(text, color8)
        } else {
            text
        }
    }

    /// Sets 8 bit color code.
    /// 0-7 are standard colors
    /// 8-15 are high intensity colors
    /// 16-231 are defined by 16 + 36 x r + 6 x g + b (0 <= r, g, b <= 5)
    /// 232-255 are grayscale from black to white in 24 steps
    fn color_8(&self, text: String, color: &u8) -> String {
        format!("\x1B[38;5;{}m{}{}", color, text, RESET_CODE)
    }

    /// Basic RGB colors.
    fn color_rgb(&self, text: String, rgb: &RgbColor) -> String {
        format!(
            "\x1B[38;2;{};{};{}m{}{}",
            rgb.red, rgb.green, rgb.blue, text, RESET_CODE
        )
    }

    /// Simplest ANSI color codes defind by AnsiColorCode enumerated type.
    fn color_code(&self, text: String, color_code: &AnsiColorCode) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_8() {
        let mut color = LineColor::new();
        let color_code = 9;
        color.color8 = Some(color_code);

        let message = "Arbitrary Text";

        let wrapped_message = color.wrap_color(String::from(message));
        let expected_message = format!("\x1B[38;5;{}m{}{}", color_code, message, RESET_CODE);
        assert_eq!(wrapped_message, expected_message);
    }

    #[test]
    fn test_color_code() {
        let mut color = LineColor::new();
        let color_code = AnsiColorCode::BrightBlack;
        color.ansi = Some(color_code);
        let message = "Arbitrary text";
        let wrapped_message = color.wrap_color(String::from(message));
        let expected_message = format!("\x1B[{}m{}{}", "90", message, RESET_CODE);
        assert_eq!(wrapped_message, expected_message);
    }

    #[test]
    fn test_color_rgb() {
        let mut color = LineColor::new();
        let rgb = RgbColor {
            red: 100,
            green: 101,
            blue: 102,
        };
        color.rgb = Some(rgb);

        let message = "Arbitrary text";
        let wrapped_message = color.wrap_color(String::from(message));
        let expected_message = format!(
            "\x1B[38;2;{};{};{}m{}{}",
            100, 101, 102, message, RESET_CODE
        );
        assert_eq!(wrapped_message, expected_message);
    }
}