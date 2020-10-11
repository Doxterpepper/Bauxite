use std::fmt;
use std::cmp::max;

struct Message<'a> {
    message: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(message: &'a str) -> Message {
        Message {
            message: message,
        }
    }

    /// Gets the max line length of the message. Max line length is defined by the longest
    /// substring before a new line character (\n).
    pub fn max_line_length(self) -> usize {
        let mut max_length = 0;
        for line in self.message.lines() {
            max_length = max(max_length, line.len())
        }
        max_length
    }
}

impl<'a> fmt::Display for Message<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("{}", self.message))
    }
}
