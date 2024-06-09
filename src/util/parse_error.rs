use std::fmt;

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

    pub fn with_file_data(&mut self, path: String, line: usize, column: usize) {
        if self.file.is_some() {
            panic!("attempt to set file data twice for ParseError");
        }
        self.file = Some(FileData { path, line, column });
    }
}

#[derive(Debug)]
pub enum ParseErrorCode {
    Internal = 1,
    Unknown = 2,
}

impl fmt::Display for ParseErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Internal => "Internal",
                Self::Unknown => "Unknown",
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
