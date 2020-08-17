pub mod line_type;

use line_type::LineType;

// Basic
pub const VERTICAL: &'static str = "│";
pub const HORIZONTAL: &'static str = "─";
pub const TOP_LEFT: &'static str = "┌";
pub const TOP_RIGHT: &'static str = "┐";
pub const BOTTOM_LEFT: &'static str = "└";
pub const BOTTOM_RIGHT: &'static str = "┘";

pub fn resolve_line_type(line_type: line_type::LineType) -> Lines {
    match line_type {
        LineType::Dotted => Lines {
            horizontal: String::from("╌"),
            vertical: String::from("╎"),
            top_right: String::from("┐"),
            top_left: String::from("┌"),
            bottom_right: String::from("┘"),
            bottom_left: String::from("└"),
        },
        LineType::Bold => Lines {
            horizontal: String::from("━"),
            vertical: String::from("┃"),
            top_right: String::from("┓"),
            top_left: String::from("┏"),
            bottom_right: String::from("┛"),
            bottom_left: String::from("┗"),
        },
        LineType::Double => Lines {
            horizontal: String::from("═"),
            vertical: String::from("║"),
            top_right: String::from("╗"),
            top_left: String::from("╔"),
            bottom_right: String::from("╝"),
            bottom_left: String::from("╚"),
        },
        LineType::Basic => Lines {
            horizontal: HORIZONTAL.to_string(),
            vertical: VERTICAL.to_string(),
            top_right: TOP_RIGHT.to_string(),
            top_left: TOP_LEFT.to_string(),
            bottom_right: BOTTOM_RIGHT.to_string(),
            bottom_left: BOTTOM_LEFT.to_string(),
        }
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
        resolve_line_type(LineType::Basic)
    }
}