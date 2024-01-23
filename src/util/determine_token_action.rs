use std::{fmt::Display, process::exit};

use log::error;

use crate::dm_preprocessor::token_handling::TokenizeState;

use super::{
    start_new_token::get_default_token_action, whitespace_char::is_first_non_whitespace_char,
};

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
}

impl Display for TokenAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenAction::StartNewToken => write!(f, "StartNewToken"),
            TokenAction::ContinueToken => write!(f, "ContinueToken"),
            TokenAction::EndToken => write!(f, "EndToken"),
            TokenAction::IsolateToken => write!(f, "IsolateToken"),
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
            if state.in_comment_any() || state.in_preprocessor() {
                return TokenAction::IsolateToken;
            }
            if !current_token.is_empty() {
                panic!(
                    "This should not occur: '{char}' \"{current_token}\" {:#?}",
                    state.line_tokens()
                )
            }
            state.set_in_quote(Some(char));
            TokenAction::IsolateToken
        }
        '#' => {
            if is_first_non_whitespace_char(state.line_tokens()) {
                state.set_in_preprocessor(true);
                return TokenAction::EndToken;
            }

            get_default_token_action(char, current_token)
        }
        '*' => {
            if state.in_comment_single() {
                return get_default_token_action(char, current_token);
            }

            if state.in_comment_multi() {
                return match current_token.chars().next_back() {
                    Some('/') => {
                        state.increment_comment_multi();
                        TokenAction::EndToken
                    }
                    _ => get_default_token_action(char, current_token),
                };
            }

            if state.is_last_token_an_escape() {
                return get_default_token_action(char, current_token);
            }

            match current_token.chars().next_back() {
                Some('/') => {
                    state.increment_comment_multi();
                    TokenAction::EndToken
                }
                _ => get_default_token_action(char, current_token),
            }
        }
        '/' => {
            if state.in_comment_multi() {
                return match current_token.chars().next_back() {
                    Some('*') => {
                        state.decrement_comment_multi();
                        TokenAction::EndToken
                    }
                    _ => get_default_token_action(char, current_token),
                };
            }

            if state.is_last_token_an_escape() {
                return get_default_token_action(char, current_token);
            }

            match current_token.chars().next_back() {
                Some('/') => {
                    state.set_comment_single(true);
                    TokenAction::EndToken
                }
                _ => get_default_token_action(char, current_token),
            }
        }
        _ => get_default_token_action(char, current_token),
    }
}
