pub mod line_type;

// Basic
pub const VERTICAL: &'static str = "│";
pub const HORIZONTAL: &'static str = "─";
pub const TOP_LEFT: &'static str = "┌";
pub const TOP_RIGHT: &'static str = "┐";
pub const BOTTOM_LEFT: &'static str = "└";
pub const BOTTOM_RIGHT: &'static str = "┘";

fn resolve_line_type(line_type: line_type::LineType) -> Lines {
    Lines {
        horizontal: HORIZONTAL.to_string(),
        vertical: VERTICAL.to_string(),
        top_right: TOP_RIGHT.to_string(),
        top_left: TOP_LEFT.to_string(),
        bottom_right: BOTTOM_RIGHT.to_string(),
        bottom_left: BOTTOM_LEFT.to_string(),
    }
}

pub struct Lines {
    pub horizontal: String,
    pub vertical: String,
    pub top_right: String,
    pub top_left: String,
    pub bottom_right: String,
    pub bottom_left: String,
}

impl Lines {
    /// Construct Lines as their default values
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
}

/*
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

    #[test]
    fn test_color_rgb() {
        let lines = Lines::new().color_rgb(255, 255, 255);

        let horizontal = format!("\x1B[38;2;255;255;255m{}\x1B[0m", HORIZONTAL);
        let vertical = format!("\x1B[38;2;255;255;255m{}\x1B[0m", VERTICAL);
        let top_right = format!("\x1B[38;2;255;255;255m{}\x1B[0m", TOP_RIGHT);
        let top_left = format!("\x1B[38;2;255;255;255m{}\x1B[0m", TOP_LEFT);
        let bottom_left = format!("\x1B[38;2;255;255;255m{}\x1B[0m", BOTTOM_LEFT);
        let bottom_right = format!("\x1B[38;2;255;255;255m{}\x1B[0m", BOTTOM_RIGHT);

        assert_eq!(lines.horizontal, horizontal);
        assert_eq!(lines.vertical, vertical);
        assert_eq!(lines.top_right, top_right);
        assert_eq!(lines.top_left, top_left);
        assert_eq!(lines.bottom_left, bottom_left);
        assert_eq!(lines.bottom_right, bottom_right);
    }
}
*/