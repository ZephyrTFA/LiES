use std::{fmt::Display, process::exit};

use crate::dm_preprocessor::token_handling::TokenizeState;

use super::{
    start_new_token::should_start_new_token, whitespace_char::is_first_non_whitespace_char,
};

#[derive(Debug, PartialEq)]
pub enum TokenAction {
    /// Start a new token with the current character.
    StartNewToken,
    /// Add the character to the current token.
    ContinueToken,
    /// Ignore the current character.
    None,
}

impl Display for TokenAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenAction::StartNewToken => write!(f, "StartNewToken"),
            TokenAction::ContinueToken => write!(f, "ContinueToken"),
            TokenAction::None => write!(f, "None"),
        }
    }
}

// Determines what action should be taken for the current character.
pub fn determine_token_action(
    state: &mut TokenizeState,
    char: char,
    current_token: &str,
) -> TokenAction {
    if let Some(quote_char) = state.in_quote() {
        if (char == *quote_char && !current_token.ends_with('\\')) {
            state.set_in_quote(None);
            return TokenAction::StartNewToken;
        } else {
            return TokenAction::ContinueToken;
        }
    }

    match char {
        '"' | '\'' => {
            if !current_token.is_empty() {
                panic!("This should not occur")
            }
            state.set_in_quote(Some(char));
            TokenAction::StartNewToken
        }
        '#' => {
            if is_first_non_whitespace_char(state.line_tokens()) {
                state.set_in_preprocessor(true);
            }

            if should_start_new_token(char, current_token) {
                TokenAction::StartNewToken
            } else {
                TokenAction::ContinueToken
            }
        }
        '.' => TokenAction::StartNewToken,
        ' ' | '\t' => {
            if current_token.ends_with(char) {
                TokenAction::ContinueToken
            } else if !current_token.is_empty() {
                TokenAction::StartNewToken
            } else {
                TokenAction::None
            }
        }
        _ => {
            if should_start_new_token(char, current_token) {
                TokenAction::StartNewToken
            } else {
                TokenAction::ContinueToken
            }
        }
    }
}
