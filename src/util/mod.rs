use std::path::PathBuf;

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
