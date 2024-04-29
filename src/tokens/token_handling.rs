use log::{error, trace};

use crate::{
    dm_preprocessor::lib::DmPreProcessor, tokens::token_action::TokenAction,
    util::condense_lines::condense_lines,
};

use super::{determine_token_action::determine_token_action, dm_token::DmToken};

impl DmPreProcessor {
    pub fn tokenize(&mut self, lines: &[impl Into<String> + Clone]) -> Vec<DmToken> {
        let condensed_lines: Vec<String> = condense_lines(lines);
        // let condensed_lines = condense_braces(&condensed_lines);
        let mut tokens: Vec<DmToken> = vec![];

        for line in condensed_lines {
            let mut token = String::new();
            self.tokenize_state.set_in_preprocessor(false);
            self.tokenize_state.set_comment_single(false);

            trace!("Tokenizing line: `{}`", line);
            let mut chars = line.chars().peekable();
            while let Some(char) = chars.next() {
                trace!("Char: `{}`", char.escape_debug());

                let next_action =
                    determine_token_action(&mut self.tokenize_state, char, &token, &mut chars);
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
            if self.tokenize_state.in_quote().is_some()
                && !self.tokenize_state.in_preprocessor()
                && !self.tokenize_state.multiline_string()
            {
                error!(
                    "Unterminated quote `{}` in line `{}`",
                    self.tokenize_state.in_quote().unwrap(),
                    line
                );
                panic!();
            }
        }

        let path = self.get_current_file();
        if self.tokenize_state.unmatched_brackets() {
            error!("Unmatched brackets in file `{}`", path.display());
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
}
