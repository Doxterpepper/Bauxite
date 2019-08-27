pub mod color;

pub const VERTICAL: &'static str = "│";
pub const HORIZONTAL: &'static str = "─";
pub const TOP_LEFT: &'static str = "┌";
pub const TOP_RIGHT: &'static str = "┐";
pub const BOTTOM_LEFT: &'static str = "└";
pub const BOTTOM_RIGHT: &'static str = "┘";

pub struct Lines {
    pub horizontal: String,
    pub vertical: String,
    pub top_right: String,
    pub top_left: String,
    pub bottom_right: String,
    pub bottom_left: String,
}

impl Lines {
    pub fn new() -> Lines {
        Lines {
            horizontal: HORIZONTAL.to_string(),
            vertical: VERTICAL.to_string(),
            top_right: TOP_RIGHT.to_string(),
            top_left: TOP_LEFT.to_string(),
            bottom_right: BOTTOM_RIGHT.to_string(),
            bottom_left: BOTTOM_LEFT.to_string(),
        }
    }

    pub fn color_8(mut self, color: u8) -> Self {
        self.horizontal = color::color_8(self.horizontal.clone(), color);
        self.vertical = color::color_8(self.vertical.clone(), color);
        self.top_right = color::color_8(self.top_right.clone(), color);
        self.top_left = color::color_8(self.top_left.clone(), color);
        self.bottom_left = color::color_8(self.bottom_left.clone(), color);
        self.bottom_right = color::color_8(self.bottom_right.clone(), color);
        self
    }

    pub fn color_rgb(mut self, red: u8, green: u8, blue: u8) -> Self {
        self.horizontal = color::color_rgb(self.horizontal.clone(), red, green, blue);
        self.vertical = color::color_rgb(self.vertical.clone(), red, green, blue);
        self.top_right = color::color_rgb(self.top_right.clone(), red, green, blue);
        self.top_left = color::color_rgb(self.top_left.clone(), red, green, blue);
        self.bottom_left = color::color_rgb(self.bottom_left.clone(), red, green, blue);
        self.bottom_right = color::color_rgb(self.bottom_right.clone(), red, green, blue);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_8() {
        let lines = Lines::new().color_8(9);

        let horizontal = format!("\x1B[38;5;9m{}\x1B[0m", HORIZONTAL);
        let vertical = format!("\x1B[38;5;9m{}\x1B[0m", VERTICAL);
        let top_right = format!("\x1B[38;5;9m{}\x1B[0m", TOP_RIGHT);
        let top_left = format!("\x1B[38;5;9m{}\x1B[0m", TOP_LEFT);
        let bottom_left = format!("\x1B[38;5;9m{}\x1B[0m", BOTTOM_LEFT);
        let bottom_right = format!("\x1B[38;5;9m{}\x1B[0m", BOTTOM_RIGHT);

        assert_eq!(lines.horizontal, horizontal);
        assert_eq!(lines.vertical, vertical);
        assert_eq!(lines.top_right, top_right);
        assert_eq!(lines.top_left, top_left);
        assert_eq!(lines.bottom_left, bottom_left);
        assert_eq!(lines.bottom_right, bottom_right);
    }
}
