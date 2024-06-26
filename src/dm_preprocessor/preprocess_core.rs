use std::collections::VecDeque;

use ::log::{error, trace};

use crate::{
    tokens::dm_token::DmToken,
    util::{dm_file::DmFile, ParseError},
};

use super::lib::DmPreProcessor;

impl DmPreProcessor {
    pub fn preprocess(&mut self, file: &DmFile) -> Result<VecDeque<DmToken>, ParseError> {
        self.tokenize_state.set_lines(file.lines());
        let mut tokens: VecDeque<DmToken> = self.start_tokenizing().into();
        let mut final_tokens: VecDeque<DmToken> = VecDeque::new();

        loop {
            if tokens.is_empty() {
                break;
            }

            let token = tokens.pop_front().unwrap();
            trace!("Token: {}", token.value().escape_debug());
            let token = if !token.is_in_string() {
                self.do_define_replacement(token, &mut tokens)
                    .map_err(|err| {
                        err.with_file_path(self.get_current_file().display().to_string())
                    })?
            } else {
                Some(token)
            };
            if token.is_none() {
                continue;
            }
            let token = token.unwrap();

            if !token.is_in_string() {
                let chars: Vec<char> = token.value().chars().collect();
                if chars.first().is_some_and(|c| c.is_whitespace())
                    && !chars.iter().all(|c| c.is_whitespace())
                {
                    error!("token not in string and contains mixed whitespace and non whitespace chars.");
                    return Err(ParseError::INTERNAL_ERROR);
                }
            }

            if !token.is_in_string() && token.value() == "#" {
                let directive = tokens.pop_front().unwrap();
                let directive = directive.value(); // needs to be seperate because of borrow checker

                let mut args = Self::take_until_match_any(&mut tokens, &["\n", "//"]);
                trace!("directive args: {args:?}");
                if !args.is_empty() {
                    if !args[0].is_only_whitespace(false) {
                        error!("somehow no whitespace after directive token");
                        return Err(ParseError::EXPECTED_DIFFERENT_TOKEN);
                    }
                    args.remove(0);
                }
                self.handle_directive(directive, &args).map_err(|err| {
                    err.with_file_path(self.get_current_file().display().to_string())
                })?;
                continue;
            }

            if self.is_skipping() {
                continue;
            }

            final_tokens.push_back(token);
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
