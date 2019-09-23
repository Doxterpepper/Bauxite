//! Wrap terminal output with boxes
//! # Example
//! ```
//! let lorem_ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
//! eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
//! nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure
//! dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
//! Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim
//! id est laborum.";
//!
//! let boxed_message = bauxite::BoxBuilder::from(lorem_ipsum).padding(3)
//!     .alignment(bauxite::Alignment::Left);
//! println!("{}", boxed_message);
//! ```

use std::cmp::max;
use std::fmt;

mod lines;
mod formatting;

use self::formatting::Formatting;

pub use self::formatting::Alignment;
pub use self::lines::color::AnsiColorCode;

pub struct BoxBuilder {
    message: String,
    format: Formatting,
    lines: lines::Lines,
}

impl BoxBuilder {
    /// Create a new boxed message
    pub fn new(message: String) -> BoxBuilder {
        BoxBuilder {
            message: message,
            format: Formatting::new(),
            lines: lines::Lines::new(),
        }
    }

    /// Create new boxed message from a str
    pub fn from(message: &str) -> BoxBuilder {
        BoxBuilder {
            message: String::from(message),
            format: Formatting::new(),
            lines: lines::Lines::new(),
        }
    }

    /// Set the global padding on the box
    pub fn padding(mut self, pad: usize) -> Self {
        self.format.padding = pad;
        self
    }

    /// Set the alignment of the content
    pub fn alignment(mut self, align: Alignment) -> Self {
        self.format.alignment = align;
        self
    }

    /// Set the maximum width of the box before lines should wrap
    pub fn max_width(mut self, width: usize) -> Self {
        self.format.max_width = width;
        self
    }

    /// Set the padding on the bottom, overrides the global bottom padding
    pub fn padding_bottom(mut self, pad: usize) -> Self {
        self.format.padding_bottom = Some(pad);
        self
    }

    /// Set the padding on the top, overrides the global top padding
    pub fn padding_top(mut self, pad: usize) -> Self {
        self.format.padding_top = Some(pad);
        self
    }

    /// Set the padding on the left, overrides the global left padding
    pub fn padding_left(mut self, pad: usize) -> Self {
        self.format.padding_left = Some(pad);
        self
    }

    /// Set the padding on the right, overrides the global right padding
    pub fn padding_right(mut self, pad: usize) -> Self {
        self.format.padding_right = Some(pad);
        self
    }

    /// Sets 8 bit color code.
    /// 0-7 are standard colors
    /// 8-15 are high intensity colors
    /// 16-231 are defined by 16 + 36 x r + 6 x g + b (0 <= r, g, b <= 5)
    /// 232-255 are grayscale from black to white in 24 steps
    pub fn color_8(mut self, color: u8) -> Self {
        self.lines = self.lines.color_8(color);
        self
    }

    /// Basic RGB colors.
    pub fn color_rgb(mut self, red: u8, green: u8, blue: u8) -> Self {
        self.lines = self.lines.color_rgb(red, green, blue);
        self
    }

    /// Simplest ANSI color codes defind by AnsiColorCode enumerated type.
    pub fn color(mut self, code: AnsiColorCode) -> Self {
        self.lines = self.lines.color_code(code);
        self
    }

    /// Boxed message to string
    pub fn to_string(&self) -> String {
        let format = &self.format;
        let right_padding = format.padding_right.unwrap_or(format.padding);
        let left_padding = format.padding_left.unwrap_or(format.padding);
        let total_horizontal_pad = right_padding + left_padding;

        let normalized_message =
            normalize_lines(&self.message, format.max_width, total_horizontal_pad);
        let max_line_length = max_line_length(&normalized_message);

        // wrap the message in the box
        let mut boxed_message = self.gen_top(max_line_length + right_padding + left_padding);
        boxed_message += &self.gen_top_padding(max_line_length + total_horizontal_pad);
        boxed_message += &self.wrap_lines(&normalized_message, max_line_length);
        boxed_message += &self.gen_bottom_padding(max_line_length + right_padding + left_padding);
        boxed_message += &self.gen_bottom(max_line_length + left_padding + right_padding);
        boxed_message
    }

    /// Helper function to build the top of the box
    fn gen_top(&self, length: usize) -> String {
        let vertical_line = (0..length)
            .map(|_| self.lines.horizontal.clone())
            .collect::<String>();
        format!(
            "{}{}{}\n",
            self.lines.top_left, vertical_line, self.lines.top_right
        )
    }

    /// Helper function to build the bottom of the box
    fn gen_bottom(&self, length: usize) -> String {
        let vertical_line = (0..length)
            .map(|_| self.lines.horizontal.clone())
            .collect::<String>();
        format!(
            "{}{}{}",
            self.lines.bottom_left, vertical_line, self.lines.bottom_right
        )
    }

    /// Wrap the message with the box on it's left and right
    fn wrap_lines(&self, message: &String, max_length: usize) -> String {
        message
            .lines()
            .map(|line| {
                let left_padding = self.gen_left_padding(line.len(), &max_length);
                let right_padding = self.gen_right_padding(line.len(), &max_length);
                format!(
                    "{}{}{}{}{}\n",
                    self.lines.vertical, left_padding, line, right_padding, self.lines.vertical
                )
            })
            .collect::<String>()
    }

    /// Helper function to to_string padding left of the content
    fn gen_left_padding(&self, line_length: usize, max_length: &usize) -> String {
        let padding = match self.format.alignment {
            Alignment::Left => self.format.padding,
            Alignment::Right => self.format.padding + max_length - line_length,
        };
        gen_whitespace(padding)
    }

    /// Helper function to to_string padding right of the content
    fn gen_right_padding(&self, line_length: usize, max_length: &usize) -> String {
        let padding = match self.format.alignment {
            Alignment::Right => self.format.padding,
            Alignment::Left => self.format.padding + max_length - line_length,
        };
        gen_whitespace(padding)
    }

    /// Helper function to to_string top and bottom padding of the box
    fn gen_top_padding(&self, length: usize) -> String {
        let top_padding = self.format.padding_top.unwrap_or(self.format.padding / 2);
        (0..top_padding)
            .map(|_| {
                format!(
                    "{}{}{}\n",
                    self.lines.vertical,
                    gen_whitespace(length),
                    self.lines.vertical
                )
            })
            .collect::<String>()
    }

    /// Helper function to to_string top and bottom padding of the box
    fn gen_bottom_padding(&self, length: usize) -> String {
        let bottom_padding = self
            .format
            .padding_bottom
            .unwrap_or(self.format.padding / 2);
        (0..bottom_padding)
            .map(|_| {
                format!(
                    "{}{}{}\n",
                    self.lines.vertical,
                    gen_whitespace(length),
                    self.lines.vertical
                )
            })
            .collect::<String>()
    }
}

impl fmt::Display for BoxBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("{}", self.to_string()))
    }
}

fn normalize_lines(message: &String, max_width: usize, padding: usize) -> String {
    let mut normalized_message = String::new();
    let mut message_lines = message.lines();
    let mut current = message_lines.next();

    while let Some(line) = current {
        if line.len() > max_width {
            let (line1, line2) = line.split_at(max_width - padding - 2);
            normalized_message += line1;
            normalized_message += "\n";
            current = Some(line2);
        } else {
            normalized_message += line;
            normalized_message += "\n";
            current = message_lines.next();
        }
    }

    // Bauxite doesn't handle the tab character very well so
    // replace all tab characters with a single space.
    normalized_message.replace("\t", " ")
}

/// Helper function to get the length of the longest line
fn max_line_length(message: &String) -> usize {
    let mut max_length = 0;
    for line in message.lines() {
        max_length = max(max_length, line.len())
    }
    max_length
}

/// Helper function to to_string whitespace for padding
fn gen_whitespace(num: usize) -> String {
    (0..num).map(|_| " ").collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nomralize_lines() {
        let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let expected = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tem\npor incididunt ut labore et dolore magna aliqua.\n";

        let normalized = normalize_lines(&String::from(message), 80, 3);
        assert_eq!(expected, normalized);
    }

    #[test]
    fn test_basic_box() {
        let expected = "┌────────────┐\n\
                        │            │\n\
                        │  whatever  │\n\
                        │  whatever  │\n\
                        │            │\n\
                        └────────────┘";
        let boxed_content = BoxBuilder::new(String::from("whatever\nwhatever"));
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_left_align() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐\n\
                        │                                                                      │\n\
                        │  Lorem ipsum dolor sit amet,                                         │\n\
                        │  consectetur adipiscing elit,                                        │\n\
                        │  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │\n\
                        │                                                                      │\n\
                        └──────────────────────────────────────────────────────────────────────┘";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = BoxBuilder::new(String::from(message)).alignment(Alignment::Left);
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_right_align() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐\n\
                        │                                                                      │\n\
                        │                                         Lorem ipsum dolor sit amet,  │\n\
                        │                                        consectetur adipiscing elit,  │\n\
                        │  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │\n\
                        │                                                                      │\n\
                        └──────────────────────────────────────────────────────────────────────┘";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = BoxBuilder::new(String::from(message)).alignment(Alignment::Right);
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_fmt() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐\n\
                        │                                                                      │\n\
                        │  Lorem ipsum dolor sit amet,                                         │\n\
                        │  consectetur adipiscing elit,                                        │\n\
                        │  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │\n\
                        │                                                                      │\n\
                        └──────────────────────────────────────────────────────────────────────┘";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = BoxBuilder::new(String::from(message)).alignment(Alignment::Left);
        assert_eq!(expected, format!("{}", boxed_content));
    }

    #[test]
    fn empty_box() {
        let expected = "┌────┐\n\
                        │    │\n\
                        │    │\n\
                        └────┘";
        let message = "";
        let boxed_content = BoxBuilder::new(String::from(message));
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn line_wrapping() {
        let expected = "┌──────────────────────────────────────────────────────────────────────────────┐\n\
                        │                                                                              │\n\
                        │  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod te  │\n\
                        │  mpor incididunt ut labore et dolore magna aliqua.                           │\n\
                        │                                                                              │\n\
                        └──────────────────────────────────────────────────────────────────────────────┘";
        let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = BoxBuilder::new(String::from(message));
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn handle_tab_character() {
        let expected = "┌──────┐\n\
                        │      │\n\
                        │      │\n\
                        │      │\n\
                        └──────┘";
        let message = "		";
        let boxed_content = BoxBuilder::new(String::from(message));
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_from() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐\n\
                        │                                                                      │\n\
                        │  Lorem ipsum dolor sit amet,                                         │\n\
                        │  consectetur adipiscing elit,                                        │\n\
                        │  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │\n\
                        │                                                                      │\n\
                        └──────────────────────────────────────────────────────────────────────┘";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = BoxBuilder::from(message).alignment(Alignment::Left);
        assert_eq!(expected, format!("{}", boxed_content));
    }
}
