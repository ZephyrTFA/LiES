use std::char;

use log::{error, trace};

mod at;
mod close_bracket;
pub mod defaults;
mod forward_slash;
mod hash;
mod open_bracket;
mod quotes;
mod star;

use at::handle_at;
use close_bracket::handle_close_bracket;
use defaults::handle_defaults;
use forward_slash::handle_forward_slash;
use hash::handle_hash;
use open_bracket::handle_open_bracket;
use quotes::handle_quotes;
use star::handle_star;

use crate::{
    dm_preprocessor::lib::DmPreProcessor, tokens::token_action::TokenAction,
    util::count_backslashes,
};

use super::dm_token::DmToken;

impl DmPreProcessor {
    /// Tokenizes the current file.
    pub fn start_tokenizing(&mut self) -> Vec<DmToken> {
        let mut tokens: Vec<DmToken> = vec![];

        while self.tokenize_state.next_line() {
            self.tokenize_state.set_in_preprocessor(false);
            self.tokenize_state.set_comment_single(false);

            let token = self.get_token();

            if !token.is_empty() {
                self.tokenize_state.add_line_token(token);
            }

            self.tokenize_state.add_line_token("\n");
            tokens.append(&mut self.tokenize_state.finalize_line_tokens());

            if self.tokenize_state.in_quote().is_some()
                && !self.tokenize_state.in_preprocessor()
                && !self.tokenize_state.multiline_string()
            {
                error!(
                    "Unterminated quote `{}` in line `{}`",
                    self.tokenize_state.in_quote().unwrap(),
                    self.tokenize_state.current_line()
                );
                panic!();
            }
        }

        let path = self.get_current_file();
        if self.tokenize_state.unmatched_brackets() {
            error!("Unmatched brackets in file `{}`", path.display());
            panic!();
        }
        if self.tokenize_state.in_comment_multi() {
            error!(
                "Unterminated multi-line comment in file `{}`",
                path.display()
            );
            panic!();
        }
        if self.tokenize_state.in_string_interop() {
            error!("Unmatched string interop in file `{}`", path.display());
            panic!();
        }
        if self.tokenize_state.multiline_string() {
            error!("Unterminated multiline string in file `{}`", path.display());
            panic!();
        }

        tokens
    }

    /// Returns the next token in the current line.
    fn get_token(&mut self) -> String {
        let mut token = String::new();
        let mut last_action = TokenAction::None;

        while let Some(char) = self.tokenize_state.next_char() {
            trace!("Char: `{}`", char.escape_debug());

            let next_action = self.get_token_action(char, &token);
            if last_action == TokenAction::DelayTokenDrop && next_action != last_action {
                token = String::new();
            }
            last_action = next_action;

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
                TokenAction::DropToken => {
                    token = String::new();
                }
                TokenAction::None => {}
                TokenAction::DelayTokenDrop => {
                    token.push(char);
                }
            }
        }

        if last_action == TokenAction::DelayTokenDrop {
            token = String::new();
        }
        token
    }

    /// Returns the action to take for the given character.
    fn get_token_action(&mut self, char: char, current_token: &str) -> TokenAction {
        if self.tokenize_state.in_string_special_escape() {
            return self.handle_string_special_escape(char);
        }

        if let Some(quote_char) = self.tokenize_state.in_quote() {
            return self.handle_string_in_quote(char, *quote_char, current_token);
        }

        if self.tokenize_state.in_comment_single() {
            return TokenAction::None; // single comments always override multi-line comment markers
        }

        if self.tokenize_state.in_comment_multi() {
            return self.handle_comment_multi(char, current_token);
        }

        let mut token_result = match char {
            ']' => handle_close_bracket(&mut self.tokenize_state, current_token),
            '[' => handle_open_bracket(&mut self.tokenize_state),
            '"' | '\'' => handle_quotes(&mut self.tokenize_state, char, current_token),
            '#' => handle_hash(&mut self.tokenize_state),
            '*' => handle_star(&mut self.tokenize_state, current_token),
            '/' => handle_forward_slash(&mut self.tokenize_state, current_token),
            '@' => handle_at(&mut self.tokenize_state),
            _ => handle_defaults(char, current_token),
        };

        if token_result == TokenAction::None {
            token_result = handle_defaults(char, current_token);
        }

        token_result
    }

    /// Edge case handling for when we are in a string.
    fn handle_string_in_quote(
        &mut self,
        char: char,
        quote_char: char,
        current_token: &str,
    ) -> TokenAction {
        self.tokenize_state.set_token_is_in_string(true);
        if char == quote_char && count_backslashes(current_token) % 2 == 0 {
            if self.tokenize_state.multiline_string() {
                return TokenAction::ContinueToken;
            }

            self.tokenize_state.set_in_quote(None);
            TokenAction::IsolateToken
        } else {
            match char {
                '[' if self.tokenize_state.in_quote() == Some(&'"')
                    && !self.tokenize_state.string_literal()
                    && count_backslashes(current_token) % 2 == 0 =>
                {
                    self.tokenize_state.increment_string_interop_count();
                    self.tokenize_state.set_next_token_is_in_string(true);
                    TokenAction::IsolateToken
                }
                '}' if self.tokenize_state.multiline_string()
                    && current_token.ends_with(quote_char) =>
                {
                    self.tokenize_state.set_in_quote(None);
                    self.tokenize_state.set_multiline_string(false);
                    TokenAction::IsolateToken
                }
                _ => TokenAction::ContinueToken,
            }
        }
    }

    /// Edge case handling for when we are in a string special escape.
    fn handle_string_special_escape(&mut self, char: char) -> TokenAction {
        if char
            != *self
                .tokenize_state
                .in_quote()
                .expect("we are in a string special escape without a quote character")
        {
            return TokenAction::ContinueToken;
        }

        if self.tokenize_state.multiline_string()
            && !self
                .tokenize_state
                .next_char_peek()
                .is_some_and(|c| *c == '}')
        {
            return TokenAction::ContinueToken;
        }

        self.tokenize_state.set_in_quote(None);
        self.tokenize_state.set_in_string_special_escape(false);
        if self.tokenize_state.multiline_string() {
            assert_eq!(
                self.tokenize_state
                    .next_char()
                    .expect("unexpected end of line in multiline string special escape"),
                '}'
            );
            self.tokenize_state.set_multiline_string(false);
        }

        TokenAction::IsolateToken
    }

    fn handle_comment_multi(&mut self, char: char, current_token: &str) -> TokenAction {
        match char {
            '*' => {
                if current_token.ends_with('/') {
                    self.tokenize_state.increment_comment_multi();
                }
                TokenAction::DelayTokenDrop
            }
            '/' => {
                if current_token.ends_with('*') {
                    self.tokenize_state.decrement_comment_multi();
                }
                TokenAction::DelayTokenDrop
            }
            _ => TokenAction::DropToken,
        }
    }
}
