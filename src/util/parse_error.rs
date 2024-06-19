use std::fmt;

use crate::{preprocess::PreprocessState, tokenize::token::Token};

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
        full_path: impl Into<&'a str>,
        line: usize,
        column: usize,
    ) -> Self {
        self.file = Some(FileData {
            path: path.into().to_string(),
            full_path: full_path.into().to_string(),
            line,
            column,
        });
        self
    }

    pub fn with_preprocessor_state(self, state: &PreprocessState, token: &Token) -> Self {
        let current_file_entry = state
            .environment()
            .current_file()
            .expect("with_preprocessor_state but no active file");
        self.with_file_data(
            current_file_entry.path(),
            current_file_entry.full_path(),
            token.line(),
            token.column(),
        )
    }

    pub fn with_preprocessor_state_if_not_set(
        self,
        state: &PreprocessState,
        token: &Token,
    ) -> Self {
        if self.file.is_some() {
            return self;
        }
        self.with_preprocessor_state(state, token)
    }
}

#[derive(Debug)]
pub enum ParseErrorCode {
    Internal = 1,
    Unknown = 2,
    UnexpectedEOL = 3,
    ExpectedString = 4,
    MalformedString = 5,
    UnknownDirective = 6,
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
                Self::UnknownDirective => "Unknown directive",
            }
        )
    }
}

#[derive(Debug)]
pub struct FileData {
    path: String,
    full_path: String,
    line: usize,
    column: usize,
}

impl FileData {
    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn full_path(&self) -> &String {
        &self.full_path
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn column(&self) -> &usize {
        &self.column
    }
}
