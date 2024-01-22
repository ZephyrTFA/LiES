use std::{fmt::Display, process::exit};

use log::{debug, error};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::{
    condense_lines::condense_lines,
    determine_token_action::{determine_token_action, TokenAction},
    start_new_token::should_start_new_token,
};

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

impl DmPreProcessor {
    pub fn tokenize(&mut self, lines: &[String]) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = condense_lines(lines);
        let mut tokens: Vec<DmToken> = vec![];
        let mut in_quote: Option<char> = None;
        let mut in_comment = false;
        let mut in_multiline_comment = false;
        let mut in_preprocessor = false;

        for line in condensed_lines {
            let mut line_tokens: Vec<DmToken> = vec![];
            let mut token = String::new();

            for char in line.chars() {
                let next_action = determine_token_action(char, &token, in_quote);
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
}
