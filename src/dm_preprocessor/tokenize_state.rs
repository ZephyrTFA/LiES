use std::collections::VecDeque;

use log::trace;

use crate::{
    tokens::dm_token::DmToken,
    util::{condense_lines::condense_lines, count_backslashes},
};

#[derive(Debug, Default)]
pub struct TokenizeState {
    current_line: String,
    remaining_lines: VecDeque<String>,
    remaining_chars: VecDeque<char>,
    in_quote: Option<char>,
    in_string_special_escape: bool,
    comment_single: bool,
    comment_multi: usize,
    in_preprocessor: bool,
    line_tokens: Vec<DmToken>,
    string_interop_count: usize,
    unmatched_brackets: Vec<usize>,
    string_literal: bool,
    multiline_string: bool,
    string_interop_buckets: VecDeque<(bool, bool, Option<char>)>,
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

    pub fn multiline_string(&self) -> bool {
        self.multiline_string
    }

    pub fn set_multiline_string(&mut self, multiline_string: bool) {
        if multiline_string != self.multiline_string {
            trace!("Setting multiline string to true");
        } else {
            trace!("Setting multiline string to false");
        }
        self.multiline_string = multiline_string;
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
        self.string_interop_buckets.push_front((
            self.multiline_string,
            self.string_literal,
            self.in_quote,
        ));
        self.multiline_string = false;
        self.string_literal = false;
        self.in_quote = None;
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
        let (multiline_string, string_literal, in_quote) =
            self.string_interop_buckets.pop_front().unwrap();
        self.multiline_string = multiline_string;
        self.string_literal = string_literal;
        self.in_quote = in_quote;
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

    pub fn set_in_string_special_escape(&mut self, in_string_special_escape: bool) {
        if in_string_special_escape {
            trace!("Setting in string special escape to true");
        } else {
            trace!("Setting in string special escape to false");
        }
        self.in_string_special_escape = in_string_special_escape;
    }

    pub fn in_string_special_escape(&self) -> bool {
        self.in_string_special_escape
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

    pub fn is_last_token(&self, chars: &[char]) -> bool {
        let last: Option<&DmToken> = self.line_tokens.last();
        if last.is_none() {
            return false;
        }
        last.unwrap().value().ends_with(chars)
    }

    pub fn next_line(&mut self) -> bool {
        if let Some(line) = self.remaining_lines.pop_front() {
            self.remaining_chars = line.chars().collect();
            self.current_line = line;
            true
        } else {
            false
        }
    }

    pub fn current_line(&self) -> &String {
        &self.current_line
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.remaining_chars.pop_front()
    }

    pub fn next_char_peek(&self) -> Option<&char> {
        self.remaining_chars.front()
    }

    pub fn remaining_chars(&self) -> &VecDeque<char> {
        &self.remaining_chars
    }

    pub fn remaining_lines(&self) -> &VecDeque<String> {
        &self.remaining_lines
    }

    pub fn set_lines(&mut self, lines: &[String]) {
        let lines = condense_lines(lines);
        self.remaining_lines = lines.into();
    }
}
