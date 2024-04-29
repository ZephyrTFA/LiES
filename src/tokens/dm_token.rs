use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct DmToken {
    pub value: String,
}

impl Display for DmToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl DmToken {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
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
