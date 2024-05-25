use std::fmt::{Display, Formatter};

use ::log::trace;

pub mod condense_lines;
pub mod dm_file;
pub mod exit_codes;
pub mod log;
pub mod whitespace_char;

#[derive(Debug)]
pub struct ParseError {
    error_code: i32,
    file_path: Option<String>,
    line_number: Option<usize>,
}

impl ParseError {
    pub const ERROR_DIRECTIVE_PARSE: ParseError = ParseError {
        error_code: 1,
        file_path: None,
        line_number: None,
    };

    pub const ERROR_FORCED: ParseError = ParseError {
        error_code: 2,
        file_path: None,
        line_number: None,
    };

    pub const ERROR_MACRO_EMPTY_BODY: ParseError = ParseError {
        error_code: 3,
        file_path: None,
        line_number: None,
    };

    pub const ERROR_MACRO_NOT_ENOUGH_ARGS: ParseError = ParseError {
        error_code: 4,
        file_path: None,
        line_number: None,
    };

    pub const ERROR_MACRO_ARG_NAME_INVALID_CHAR: ParseError = ParseError {
        error_code: 5,
        file_path: None,
        line_number: None,
    };
    pub const ERROR_MACRO_MALFORMED_CALL: ParseError = ParseError {
        error_code: 6,
        file_path: None,
        line_number: None,
    };
    pub const DM_FILE_LOAD_FAILURE: ParseError = ParseError {
        error_code: 7,
        file_path: None,
        line_number: None,
    };
    pub const PATH_CANONICALIZE_FAIL: ParseError = ParseError {
        error_code: 8,
        file_path: None,
        line_number: None,
    };
    pub const ERROR_MACRO_MALFORMED_ARGUMENTS: ParseError = ParseError {
        error_code: 9,
        file_path: None,
        line_number: None,
    };
    pub const ERROR_MACRO_TOO_MANY_ARGS: ParseError = ParseError {
        error_code: 10,
        file_path: None,
        line_number: None,
    };
    pub const ERROR_MACRO_TOO_FEW_ARGS: ParseError = ParseError {
        error_code: 11,
        file_path: None,
        line_number: None,
    };
    pub const INTERNAL_ERROR: ParseError = ParseError {
        error_code: 0,
        file_path: None,
        line_number: None,
    };
}

impl ParseError {
    pub fn new(error_code: i32) -> Self {
        Self {
            error_code,
            file_path: None,
            line_number: None,
        }
    }

    pub fn with_file_path(mut self, file_path: String) -> Self {
        if self.file_path.is_none() {
            self.file_path = Some(file_path);
        }
        self
    }

    pub fn with_file_path_override(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }

    pub fn with_line_number(mut self, line_number: usize) -> Self {
        if self.line_number.is_none() {
            self.line_number = Some(line_number);
        }
        self
    }

    pub fn with_line_number_override(mut self, line_number: usize) -> Self {
        self.line_number = Some(line_number);
        self
    }

    pub fn line_number(&self) -> Option<usize> {
        self.line_number
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fail_reason = match self.error_code {
            0 => "Internal error",
            1 => "Failed to parse directive",
            2 => "Forced error",
            3 => "Macro body is empty",
            4 => "Macro does not have enough arguments",
            5 => "Macro argument name contains invalid characters",
            6 => "Macro call is malformed",
            7 => "Failed to load DM file",
            8 => "Failed to canonicalize path",
            9 => "Macro arguments are malformed",
            10 => "Macro has too many arguments",
            11 => "Macro has too few arguments",
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

pub fn is_valid_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub fn is_valid_ident_char_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_valid_identifier(string: &str) -> bool {
    if string.is_empty() {
        trace!("Empty identifier");
        return false;
    }
    let mut chars = string.chars();
    if !is_valid_ident_char_start(chars.next().unwrap()) {
        trace!(
            "Invalid identifier start character: {}",
            string.chars().next().unwrap()
        );
        return false;
    }
    for c in chars {
        if !is_valid_ident_char(c) {
            trace!("Invalid identifier character: {}", c);
            return false;
        }
    }
    true
}
