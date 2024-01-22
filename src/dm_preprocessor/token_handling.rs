use std::{fmt::Display, process::exit};

use log::{debug, error};
use once_cell::sync::Lazy;
use regex::Regex;

use super::DmPreProcessor;

#[derive(Debug, Clone)]
pub struct DmToken {
    value: String,
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

impl PartialEq for DmToken {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug, PartialEq)]
enum TokenAction {
    StartNewToken,
    ContinueToken,
    EndToken,
    ChangeQuoteState(Option<char>),
    None,
}

impl DmPreProcessor {
    pub fn tokenize(&mut self, lines: &[String]) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = Self::condense_lines(lines);
        let mut tokens: Vec<DmToken> = vec![];

        let mut in_quote: Option<char> = None;
        for line in condensed_lines {
            let mut token = String::new();
            for char in line.chars() {
                let next_action = Self::determine_token_action(char, &token, in_quote);
                match next_action {
                    TokenAction::StartNewToken => {
                        if !token.is_empty() {
                            tokens.push(DmToken::new(token));
                        }
                        token = char.to_string();
                    }
                    TokenAction::ContinueToken => {
                        token.push(char);
                    }
                    TokenAction::EndToken => {
                        tokens.push(DmToken::new(token));
                        token = String::new();
                    }
                    TokenAction::ChangeQuoteState(quote) => {
                        in_quote = quote;
                    }
                    _ => {
                        error!(
                            "Unexpected token action `{:?}` with char {}",
                            next_action, char
                        );
                        exit(1);
                    }
                }
            }

            if !token.is_empty() {
                tokens.push(DmToken::new(token));
            }
            tokens.push(DmToken::new("\n".into()));
        }

        tokens
    }

    // Condenses all lines that end with a backslash into a single line.
    pub fn condense_lines(lines: &[String]) -> Vec<String> {
        if lines.is_empty() {
            return vec![];
        }

        let mut condensed = vec![];
        let mut current_line = String::new();

        for line in lines {
            if line.ends_with('\\') {
                current_line.push_str(&line[..line.len() - 1]);
            } else {
                current_line.push_str(line);
                condensed.push(std::mem::take(&mut current_line));
            }
        }

        if !current_line.is_empty() {
            condensed.push(current_line);
        }

        condensed
    }

    // Determines what action should be taken for the current character.
    fn determine_token_action(
        char: char,
        current_token: &str,
        in_quote: Option<char>,
    ) -> TokenAction {
        match in_quote {
            Some(quote_char) => {
                if char == quote_char && !current_token.ends_with('\\') {
                    TokenAction::ChangeQuoteState(None)
                } else {
                    TokenAction::ContinueToken
                }
            }
            None => match char {
                '"' | '\'' => {
                    if !current_token.is_empty() {
                        return TokenAction::StartNewToken;
                    }
                    TokenAction::ChangeQuoteState(Some(char))
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
                    if Self::should_start_new_token(char, current_token) {
                        TokenAction::StartNewToken
                    } else {
                        TokenAction::ContinueToken
                    }
                }
            },
        }
    }

    // Determines if a new token should be started based on the current character and the current token.
    pub fn should_start_new_token(char: char, current_token: &str) -> bool {
        if current_token.ends_with('\\') || current_token.is_empty() {
            return false;
        }

        let is_special_symbol = matches!(
            char,
            '"' | '\'' | '+' | '-' | '*' | '/' | '%' | '^' | '&' | '|' | '=' | '<' | '>' | '!'
        );
        if is_special_symbol {
            return true;
        }

        if char.is_whitespace() {
            return !current_token.ends_with(char);
        }

        if current_token.ends_with(char::is_whitespace) {
            return true;
        }

        let is_digit_transition =
            char.is_ascii_digit() != current_token.chars().all(|c| c.is_ascii_digit());
        if is_digit_transition {
            return true;
        }

        false
    }
}
