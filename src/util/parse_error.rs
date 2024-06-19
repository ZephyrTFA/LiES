use std::fmt;

use crate::tokenize::token::Token;

#[derive(Debug)]
pub struct ParseError {
    code: ParseErrorCode,
    file: Option<FileData>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError({})", self.code)
    }
}

impl ParseError {
    pub fn new(code: ParseErrorCode) -> Self {
        Self { code, file: None }
    }

    pub fn code(&self) -> &ParseErrorCode {
        &self.code
    }

    pub fn file_data(&self) -> Option<&FileData> {
        self.file.as_ref()
    }

    pub fn with_file_data<'a>(
        mut self,
        path: impl Into<&'a str>,
        line: usize,
        column: usize,
    ) -> Self {
        if self.file.is_some() {
            panic!("attempt to set file data twice for ParseError");
        }
        self.file = Some(FileData {
            path: path.into().to_string(),
            line,
            column,
        });
        self
    }

    pub fn with_token<'a>(self, path: impl Into<&'a str>, token: &Token) -> Self {
        self.with_file_data(path, token.line(), token.column())
    }
}

#[derive(Debug)]
pub enum ParseErrorCode {
    Internal = 1,
    Unknown = 2,
    UnexpectedEOL = 3,
    ExpectedString = 4,
    MalformedString = 5,
}

impl fmt::Display for ParseErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Internal => "Internal",
                Self::Unknown => "Unknown",
                Self::UnexpectedEOL => "Unexpected end of line",
                Self::ExpectedString => "Expected string",
                Self::MalformedString => "Malformed string",
            }
        )
    }
}

#[derive(Debug)]
pub struct FileData {
    path: String,
    line: usize,
    column: usize,
}

impl FileData {
    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn column(&self) -> &usize {
        &self.column
    }
}
