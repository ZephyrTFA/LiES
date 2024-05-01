use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct DmToken {
    value: String,
    is_in_string: bool,
    line: Option<usize>,
    column: Option<usize>,
}

impl Display for DmToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl DmToken {
    pub fn new(value: String) -> Self {
        Self {
            value,
            is_in_string: false,
            line: None,
            column: None,
        }
    }

    pub fn with_is_in_string(mut self, is_in_string: bool) -> Self {
        self.is_in_string = is_in_string;
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = Some(line);
    }

    pub fn set_column(&mut self, column: usize) {
        self.column = Some(column);
    }

    pub fn is_in_string(&self) -> bool {
        self.is_in_string
    }
}

impl From<&str> for DmToken {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}

impl From<&char> for DmToken {
    fn from(value: &char) -> Self {
        Self::new(value.to_string())
    }
}

impl From<char> for DmToken {
    fn from(value: char) -> Self {
        Self::new(value.to_string())
    }
}

impl From<String> for DmToken {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl PartialEq for DmToken {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
