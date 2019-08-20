use std::cmp::max;

const VERTICAL: &'static str = "│";
const HORIZONTAL: &'static str = "─";
const TOP_LEFT: &'static str = "┌";
const TOP_RIGHT: &'static str = "┐";
const BOTTOM_LEFT: &'static str = "└";
const BOTTOM_RIGHT: &'static str = "┘";

pub fn box_content(message: String) -> String {
    let mut boxed_message = String::new();
    let max_length = max_line_length(&message);
    let margin: usize = 2;
    boxed_message = boxed_message + &gen_top(max_length + margin * 2);
    boxed_message = boxed_message + &wrap_lines(message, margin, max_length);
    boxed_message = boxed_message + &gen_bottom(max_length + margin * 2);
    boxed_message
}

fn gen_top(length: usize) -> String {
    let mut top = String::from(TOP_LEFT);
    top += &(0..length).map(|_| { HORIZONTAL }).collect::<String>();
    top += TOP_RIGHT;
    top += "\n";

    top
}

fn gen_bottom(length: usize) -> String {
    let mut bottom = String::from(BOTTOM_LEFT);
    bottom += &(0..length).map(|_| { HORIZONTAL }).collect::<String>();
    bottom += BOTTOM_RIGHT;
    bottom += "\n";

    bottom
}

fn wrap_lines(message: String, padding: usize, max_length: usize) -> String {
    let mut wrapped = String::new();
    for line in message.lines() {
        wrapped += VERTICAL;
        wrapped += &gen_whitespace(padding);
        wrapped += line;
        wrapped += &gen_whitespace(padding + max_length - line.len());
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
    let mut whitespace = String::new();
    for _ in 0..num {
        whitespace = whitespace + " ";
    }
    whitespace
}

#[test]
fn test_box_content() {
    let expected = "┌────────────┐\n│  whatever  │\n│  whatever  │\n└────────────┘\n";;
    let result = box_content(String::from("whatever\nwhatever"));
    assert_eq!(expected, result);
}
