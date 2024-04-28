use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenAction {
    /// Start a new token with the current character.
    StartNewToken,
    /// Add the character to the current token.
    ContinueToken,
    /// Add the character to the current token and end the token.
    EndToken,
    /// Isolate the current character into its own token.
    IsolateToken,
    /// Ignore the current character.
    None,
    /// Drop the current token and character.
    DropToken,
}

impl Display for TokenAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenAction::StartNewToken => write!(f, "StartNewToken"),
            TokenAction::ContinueToken => write!(f, "ContinueToken"),
            TokenAction::EndToken => write!(f, "EndToken"),
            TokenAction::IsolateToken => write!(f, "IsolateToken"),
            TokenAction::None => write!(f, "None"),
            TokenAction::DropToken => write!(f, "DropToken"),
        }
    }
}
