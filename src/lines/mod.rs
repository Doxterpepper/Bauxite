pub mod line_type;

use line_type::LineType;

/// Map a LineType value to the appropriate type of lines that should ultimatly be displayed.
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
            horizontal: String::from("─"),
            vertical: String::from("│"),
            top_right: String::from("┐"),
            top_left: String::from("┌"),
            bottom_right: String::from("┘"),
            bottom_left: String::from("└"),
        },
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
