use std::{borrow::BorrowMut, fmt::Display, process::exit};

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

#[derive(Debug, Default)]
pub struct TokenizeState<'a> {
    in_quote: Option<char>,
    in_comment: Option<InComment>,
    in_preprocessor: bool,
    line_tokens: &'a [DmToken],
}

impl TokenizeState<'_> {
    pub fn in_quote(&self) -> Option<&char> {
        self.in_quote.as_ref()
    }

    pub fn in_comment(&self) -> Option<&InComment> {
        self.in_comment.as_ref()
    }

    pub fn in_preprocessor(&self) -> bool {
        self.in_preprocessor
    }

    pub fn line_tokens(&self) -> &[DmToken] {
        self.line_tokens
    }

    pub fn set_in_quote(&mut self, quote: Option<char>) {
        self.in_quote = quote;
    }

    pub fn set_in_comment(&mut self, comment: Option<InComment>) {
        self.in_comment = comment;
    }

    pub fn set_in_preprocessor(&mut self, in_preprocessor: bool) {
        self.in_preprocessor = in_preprocessor;
    }
}

#[derive(Debug, PartialEq)]
enum InComment {
    SingleLine = 1,
    MultiLine = 2,
}

impl DmPreProcessor<'_> {
    pub fn tokenize(&mut self, lines: &[String]) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = condense_lines(lines);
        let mut tokens: Vec<DmToken> = vec![];

        for line in condensed_lines {
            let mut line_tokens: Vec<DmToken> = vec![];
            let mut token = String::new();
            self.tokenize_state.set_in_preprocessor(false);

            for char in line.chars() {
                let next_action =
                    determine_token_action(self.tokenize_state.borrow_mut(), char, &token);

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
