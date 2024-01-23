use std::{fmt::Display};

use log::{error, trace};



use crate::util::{
    condense_lines::condense_lines,
    count_backslashes,
    determine_token_action::{determine_token_action, TokenAction},
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

#[derive(Debug, Default)]
pub struct TokenizeState {
    in_quote: Option<char>,
    comment_single: bool,
    comment_multi: usize,
    in_preprocessor: bool,
    line_tokens: Vec<DmToken>,
    string_interop_count: usize,
    unmatched_brackets: Vec<usize>,
    string_literal: bool,
}

impl TokenizeState {
    pub fn string_literal(&self) -> bool {
        self.string_literal
    }

    pub fn set_string_literal(&mut self, string_literal: bool) {
        if string_literal != self.string_literal {
            trace!("Setting string literal to true");
        } else {
            trace!("Setting string literal to false");
        }
        self.string_literal = string_literal;
    }

    pub fn unmatched_brackets(&self) -> bool {
        if self.unmatched_brackets.is_empty() {
            return false;
        }
        self.unmatched_brackets.last().unwrap() > &0
    }

    pub fn increment_unmatched_brackets(&mut self) {
        let value = if self.unmatched_brackets.is_empty() {
            1
        } else {
            self.unmatched_brackets.pop().unwrap() + 1
        };
        self.unmatched_brackets.push(value);
        trace!(
            "Incrementing unmatched brackets to {}",
            self.unmatched_brackets.last().unwrap()
        );
    }

    pub fn decrement_unmatched_brackets(&mut self) {
        let value = self.unmatched_brackets.pop().unwrap() - 1;
        self.unmatched_brackets.push(value);
        trace!(
            "Decrementing unmatched brackets to {}",
            self.unmatched_brackets.last().unwrap()
        );
    }

    pub fn increment_string_interop_count(&mut self) {
        self.unmatched_brackets.push(0);
        self.string_interop_count += 1;
        trace!(
            "Incrementing string interop count to {}",
            self.string_interop_count
        );
    }

    pub fn decrement_string_interop_count(&mut self) {
        if self.unmatched_brackets.pop().unwrap() != 0 {
            panic!("Unmatched brackets in string interop");
        }
        self.string_interop_count -= 1;
        trace!(
            "Decrementing string interop count to {}",
            self.string_interop_count
        );
    }

    pub fn in_string_interop(&self) -> bool {
        self.string_interop_count > 0
    }

    pub fn in_comment_single(&self) -> bool {
        self.comment_single
    }

    pub fn in_comment_multi(&self) -> bool {
        self.comment_multi > 0
    }

    pub fn in_comment_any(&self) -> bool {
        self.in_comment_single() || self.in_comment_multi()
    }

    pub fn in_quote(&self) -> Option<&char> {
        self.in_quote.as_ref()
    }

    pub fn in_preprocessor(&self) -> bool {
        self.in_preprocessor
    }

    pub fn line_tokens(&self) -> &[DmToken] {
        &self.line_tokens
    }

    pub fn set_in_quote(&mut self, quote: Option<char>) {
        if quote.is_some() != self.in_quote.is_some() {
            trace!("Setting quote to {:?}", quote);
        }
        self.in_quote = quote;
    }

    pub fn set_in_preprocessor(&mut self, in_preprocessor: bool) {
        if in_preprocessor != self.in_preprocessor {
            trace!("Setting in preprocessor to true");
        } else {
            trace!("Setting in preprocessor to false");
        }
        self.in_preprocessor = in_preprocessor;
    }

    pub fn finalize_line_tokens(&mut self) -> Vec<DmToken> {
        let mut line_tokens = vec![];
        std::mem::swap(&mut line_tokens, &mut self.line_tokens);
        line_tokens
    }

    pub fn add_line_token(&mut self, token: impl Into<DmToken>) {
        let token = token.into();
        trace!("Token: '{}'", token.value.escape_debug());
        self.line_tokens.push(token);
    }

    pub fn set_comment_single(&mut self, comment_single: bool) {
        if comment_single != self.comment_single {
            trace!("Setting comment single to true");
        } else {
            trace!("Setting comment single to false");
        }
        self.comment_single = comment_single;
    }

    pub fn increment_comment_multi(&mut self) {
        self.comment_multi += 1;
        trace!("Incrementing comment multi to {}", self.comment_multi);
    }

    pub fn decrement_comment_multi(&mut self) {
        self.comment_multi -= 1;
        trace!("Decrementing comment multi to {}", self.comment_multi);
    }

    pub fn is_last_token_an_escape(&self) -> bool {
        let last = self.line_tokens.last();
        if last.is_none() {
            return false;
        }
        count_backslashes(last.unwrap().value()) % 2 == 1
    }
}

impl DmPreProcessor {
    pub fn tokenize(&mut self, lines: &[impl Into<String> + Clone]) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = condense_lines(lines);
        let mut tokens: Vec<DmToken> = vec![];

        for line in condensed_lines {
            let mut token = String::new();
            self.tokenize_state.set_in_preprocessor(false);
            self.tokenize_state.set_comment_single(false);

            trace!("Tokenizing line: `{}`", line);
            for char in line.chars() {
                trace!("Char: `{}`", char.escape_debug());
                let next_action = determine_token_action(&mut self.tokenize_state, char, &token);
                match next_action {
                    TokenAction::StartNewToken => {
                        if !token.is_empty() {
                            self.tokenize_state.add_line_token(token);
                        }
                        token = char.to_string();
                    }
                    TokenAction::ContinueToken => {
                        token.push(char);
                    }
                    TokenAction::EndToken => {
                        token.push(char);
                        self.tokenize_state.add_line_token(token);
                        token = String::new();
                    }
                    TokenAction::IsolateToken => {
                        if !token.is_empty() {
                            self.tokenize_state.add_line_token(token);
                        }
                        self.tokenize_state.add_line_token(char.to_string());
                        token = String::new();
                    }
                    _ => {
                        error!(
                            "Unexpected token action `{}` with char {}",
                            next_action, char
                        );
                        panic!();
                    }
                }
            }

            if !token.is_empty() {
                self.tokenize_state.add_line_token(token);
            }
            self.tokenize_state.add_line_token("\n");
            tokens.append(&mut self.tokenize_state.finalize_line_tokens());
            if self.tokenize_state.in_quote().is_some() && !self.tokenize_state.in_preprocessor() {
                error!(
                    "Unterminated quote `{}` in line `{}`",
                    self.tokenize_state.in_quote().unwrap(),
                    line
                );
                panic!();
            }

            if self.tokenize_state.unmatched_brackets() {
                error!("Unmatched brackets in line `{}`", line);
                panic!();
            }

            if self.tokenize_state.in_string_interop() {
                error!("Unmatched string interop in line `{}`", line);
                panic!();
            }
        }

        tokens
    }
}
