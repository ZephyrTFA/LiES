use std::{collections::VecDeque, process::exit};

use log::error;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    tokens::dm_token::DmToken,
    util::{dm_file::DmFile, exit_codes::ERROR_CODE_PATTERN_NOT_FOUND},
};

use super::lib::DmPreProcessor;

impl DmPreProcessor {
    pub fn preprocess(&mut self, file: &DmFile) -> Vec<DmToken> {
        self.tokenize_state.set_lines(file.lines());
        let mut tokens: VecDeque<DmToken> = self.start_tokenizing().into();
        let mut skip_until_regex: Option<Regex> = None;

        let mut in_quote: Option<char> = None;

        let mut final_tokens: Vec<DmToken> = vec![];
        loop {
            if tokens.is_empty() {
                break;
            }

            let token = tokens.pop_front().unwrap();
            let token = if in_quote.is_none() {
                self.do_define_replacement(token, &mut tokens)
            } else {
                Some(token)
            };

            if token.is_none() {
                continue;
            }
            let token = token.unwrap();

            let token = token.value();

            if let Some(until_token) = &skip_until_regex {
                if until_token.is_match(token) {
                    skip_until_regex = None;
                }
                continue;
            }

            if token.starts_with("//") {
                Self::take_until_match(&mut tokens, "\n");
                continue;
            }

            static MULTI_LINE_COMMENT_START: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"/+\*+").unwrap());
            static MULTI_LINE_COMMENT_END: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"\*+/+").unwrap());
            if MULTI_LINE_COMMENT_START.is_match(token) && !MULTI_LINE_COMMENT_END.is_match(token) {
                Self::take_until_regex(&mut tokens, &MULTI_LINE_COMMENT_END);
                continue;
            }

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

                match self.handle_directive(directive, &args) {
                    Ok(()) => {}
                    Err(code) => {
                        error!(
                            "PreProcessor Error ({code}): {directive} with the following args: {args:#?}"
                        );
                        panic!();
                    }
                }
                continue;
            }

            if self.is_skipping() {
                continue;
            }

            if token == "\"" || token == "'" {
                in_quote = Some(token.chars().next().unwrap());
                continue;
            }

            final_tokens.push(DmToken::new(token.to_owned()));
        }

        final_tokens
    }

    fn take_until_match(tokens: &mut VecDeque<DmToken>, pattern: &str) -> Vec<DmToken> {
        match Self::take_until(tokens, |token| token.value() == pattern) {
            Some(tokens) => tokens,
            None => {
                error!("Failed to find pattern `{}`", pattern);
                panic!();
            }
        }
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

    fn take_until_regex(tokens: &mut VecDeque<DmToken>, pattern: &Regex) -> Vec<DmToken> {
        match Self::take_until(tokens, |token| pattern.is_match(token.value())) {
            Some(tokens) => tokens,
            None => {
                error!(
                    "Failed to find regex pattern `{}` with remaining: `{:#?}`",
                    pattern, tokens
                );
                exit(ERROR_CODE_PATTERN_NOT_FOUND);
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
