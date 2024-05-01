use std::collections::VecDeque;

use log::{debug, error};

use crate::{
    tokens::dm_token::DmToken,
    util::{dm_file::DmFile, ParseError},
};

use super::lib::DmPreProcessor;

impl DmPreProcessor {
    pub fn preprocess(&mut self, file: &DmFile) -> Result<Vec<DmToken>, ParseError> {
        self.tokenize_state.set_lines(file.lines());
        let mut tokens: VecDeque<DmToken> = self.start_tokenizing().into();

        let mut in_quote: Option<char> = None;

        let mut final_tokens: Vec<DmToken> = vec![];
        loop {
            if tokens.is_empty() {
                break;
            }

            let token = tokens.pop_front().unwrap();
            debug!("Token: {}", token.value().escape_debug());
            let token = if in_quote.is_none() {
                self.do_define_replacement(token, &mut tokens)
                    .map_err(|err| err.with_file_path(file.path().display().to_string()))?
            } else {
                Some(token)
            };

            if token.is_none() {
                continue;
            }
            let token = token.unwrap();

            let token = token.value();

            if in_quote.is_none() && token == "#" {
                let directive = tokens.pop_front().unwrap();
                let directive = directive.value(); // needs to be seperate because of borrow checker

                let mut args = Self::take_until_match_any(&mut tokens, &["\n", "//"]);
                while !args.is_empty() {
                    if args[0].value().chars().all(char::is_whitespace) {
                        args.remove(0);
                    } else if args
                        .last()
                        .unwrap()
                        .value()
                        .chars()
                        .all(char::is_whitespace)
                    {
                        args.pop();
                    } else {
                        break;
                    }
                }

                self.handle_directive(directive, &args)
                    .map_err(|err| err.with_file_path(file.path().to_string_lossy().to_string()))?;
                continue;
            }

            if self.is_skipping() {
                continue;
            }

            if token == "\"" || token == "'" {
                in_quote = Some(token.chars().next().unwrap());
                continue;
            }

            final_tokens.push(DmToken::new(token.to_owned()).with_is_in_string(in_quote.is_some()));
        }

        Ok(final_tokens)
    }

    fn take_until_match_any(tokens: &mut VecDeque<DmToken>, patterns: &[&str]) -> Vec<DmToken> {
        match Self::take_until(tokens, |token| patterns.contains(&token.value())) {
            Some(tokens) => tokens,
            None => {
                error!("Failed to find pattern `{}`", patterns.join("`, `"));
                panic!();
            }
        }
    }
    fn take_until(
        tokens: &mut VecDeque<DmToken>,
        check: impl Fn(&DmToken) -> bool,
    ) -> Option<Vec<DmToken>> {
        let mut final_tokens = vec![];

        while !tokens.is_empty() {
            let token = tokens.pop_front().unwrap();
            if check(&token) {
                return Some(final_tokens);
            }
            final_tokens.push(token);
        }

        None
    }
}
