use std::process::exit;

use log::{debug, error};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::{dm_file::DmFile, exit_codes::ERROR_CODE_PATTERN_NOT_FOUND};

use super::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub fn preprocess(&mut self, file: &DmFile) -> Vec<DmToken> {
        let mut tokens = self.tokenize(file.lines());
        let mut skip_until_regex: Option<Regex> = None;

        let mut in_quote = None;

        let mut final_tokens: Vec<DmToken> = vec![];
        loop {
            if tokens.is_empty() {
                break;
            }

            let token = tokens.remove(0);
            let token = token.value();

            if let Some(until_token) = &skip_until_regex {
                if until_token.is_match(token) {
                    skip_until_regex = None;
                    debug!("found token match");
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
                let directive = tokens.remove(0);
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
                self.handle_directive(directive, args).unwrap();
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

    fn take_until_match(tokens: &mut Vec<DmToken>, pattern: &str) -> Vec<DmToken> {
        match Self::take_until(tokens, |token| token.value() == pattern) {
            Some(tokens) => tokens,
            None => {
                error!("Failed to find pattern `{}`", pattern);
                exit(ERROR_CODE_PATTERN_NOT_FOUND);
            }
        }
    }

    fn take_until_match_any(tokens: &mut Vec<DmToken>, patterns: &[&str]) -> Vec<DmToken> {
        match Self::take_until(tokens, |token| patterns.contains(&token.value())) {
            Some(tokens) => tokens,
            None => {
                error!("Failed to find pattern `{}`", patterns.join("`, `"));
                exit(ERROR_CODE_PATTERN_NOT_FOUND);
            }
        }
    }

    fn take_until_regex(tokens: &mut Vec<DmToken>, pattern: &Regex) -> Vec<DmToken> {
        match Self::take_until(tokens, |token| pattern.is_match(token.value())) {
            Some(tokens) => tokens,
            None => {
                error!("Failed to find regex pattern `{}`", pattern);
                exit(ERROR_CODE_PATTERN_NOT_FOUND);
            }
        }
    }

    fn take_until(
        tokens: &mut Vec<DmToken>,
        check: impl Fn(&DmToken) -> bool,
    ) -> Option<Vec<DmToken>> {
        let mut final_tokens = vec![];

        while !tokens.is_empty() {
            let token = tokens.remove(0);
            if check(&token) {
                return Some(final_tokens);
            }
            final_tokens.push(token);
        }

        None
    }
}
