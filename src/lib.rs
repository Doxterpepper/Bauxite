use std::cmp::max;

const VERTICAL: &'static str = "│";
const HORIZONTAL: &'static str = "─";
const TOP_LEFT: &'static str = "┌";
const TOP_RIGHT: &'static str = "┐";
const BOTTOM_LEFT: &'static str = "└";
const BOTTOM_RIGHT: &'static str = "┘";

pub enum Alignment {
    Left,
    Right,
}

struct Formatting {
    pub padding: usize,
    pub alignment: Alignment,
}

pub struct Bauxite {
    message: String,
    format: Formatting,
}

impl Formatting {
    pub fn new() -> Formatting {
        Formatting {
            padding: 2,
            alignment: Alignment::Left,
        }
    }
}

impl Bauxite {
    pub fn new(message: String) -> Bauxite {
        Bauxite {
            message: message,
            format: Formatting::new(),
        }
    }

    pub fn padding(mut self, pad: usize) -> Self {
        self.format.padding = pad;
        self
    }

    pub fn alignment(mut self, align: Alignment) -> Self {
        self.format.alignment = align;
        self
    }

    pub fn to_string(self) -> String {
        let max_length = max_line_length(&self.message);
        let mut boxed_message = gen_top(max_length + self.format.padding * 2);
        boxed_message = boxed_message + &wrap_lines(self.message, &self.format, max_length);
        boxed_message = boxed_message + &gen_bottom(max_length + self.format.padding * 2);
        boxed_message
    }
}

fn gen_top(length: usize) -> String {
    let mut top = String::from(TOP_LEFT);
    top += &(0..length).map(|_| HORIZONTAL).collect::<String>();
    top += TOP_RIGHT;
    top += "\n";
    top
}

fn gen_bottom(length: usize) -> String {
    let mut bottom = String::from(BOTTOM_LEFT);
    bottom += &(0..length).map(|_| HORIZONTAL).collect::<String>();
    bottom += BOTTOM_RIGHT;
    bottom += "\n";
    bottom
}

fn wrap_lines(message: String, format: &Formatting, max_length: usize) -> String {
    let mut wrapped = String::new();
    for line in message.lines() {
        let padding_left = match format.alignment {
            Alignment::Left => format.padding,
            Alignment::Right => format.padding + max_length - line.len(),
        };

        let padding_right = match format.alignment {
            Alignment::Left => format.padding + max_length - line.len(),
            Alignment::Right => format.padding,
        };

        wrapped += VERTICAL;
        wrapped += &gen_whitespace(padding_left);
        wrapped += line;
        wrapped += &gen_whitespace(padding_right);
        wrapped += VERTICAL;
        wrapped += "\n";
    }
    wrapped
}

fn max_line_length(message: &String) -> usize {
    let mut max_length = 0;
    for line in message.lines() {
        max_length = max(max_length, line.len())
    }
    max_length
}

fn gen_whitespace(num: usize) -> String {
    (0..num).map(|_| " ").collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_box() {
        let expected = "┌────────────┐
│  whatever  │
│  whatever  │
└────────────┘\n";
        let boxed_content = Bauxite::new(String::from("whatever\nwhatever"));
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_left_align() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐
│  Lorem ipsum dolor sit amet,                                         │
│  consectetur adipiscing elit,                                        │
│  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │
└──────────────────────────────────────────────────────────────────────┘\n";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = Bauxite::new(String::from(message));
        assert_eq!(expected, boxed_content.to_string());
    }

    #[test]
    fn test_right_align() {
        let expected = "┌──────────────────────────────────────────────────────────────────────┐
│                                         Lorem ipsum dolor sit amet,  │
│                                        consectetur adipiscing elit,  │
│  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  │
└──────────────────────────────────────────────────────────────────────┘\n";
        let message = "Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let boxed_content = Bauxite::new(String::from(message)).alignment(Alignment::Right);
        assert_eq!(expected, boxed_content.to_string());
    }
}
