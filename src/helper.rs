/// Helper functions to facilitate line box formatting
use std::cmp::max;

/// Set a uniform line length. Line length is no more than max_width.
pub fn normalize_lines(message: &String, max_width: usize, padding: usize) -> String {
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
pub fn max_line_length(message: &String) -> usize {
    let mut max_length = 0;
    for line in message.lines() {
        max_length = max(max_length, line.len())
    }
    max_length
}

/// Helper function to get whitespace for padding
pub fn gen_whitespace(num: usize) -> String {
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
}
