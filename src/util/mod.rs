use std::fmt::{Display, Formatter};

pub mod condense_brackets;
pub mod condense_lines;
pub mod dm_file;
pub mod exit_codes;
pub mod log;
pub mod whitespace_char;

#[derive(Debug)]
pub struct ParseError(i32);

impl ParseError {
    pub const ERROR_DIRECTIVE_PARSE: ParseError = ParseError(1);
    pub const ERROR_FORCED: ParseError = ParseError(2);
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fail_reason = match self.0 {
            1 => "Failed to parse directive",
            2 => "Forced error",
            _ => "Unknown error",
        };
        write!(f, "{}", fail_reason)
    }
}

pub fn count_backslashes(string: &str) -> usize {
    let mut count = 0;
    for char in string.chars().rev() {
        if char == '\\' {
            count += 1;
        } else {
            break;
        }
    }
    count
}
