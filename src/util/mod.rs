use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

pub mod define_definition;
pub mod dm_file;
pub mod exit_codes;
pub mod log;

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
