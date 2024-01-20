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

            if token.ends_with("//") {
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

            if token == "#" {
                let directive = tokens.remove(0);
                let directive = directive.value(); // needs to be seperate because of borrow checker
                let args = Self::take_until_match(&mut tokens, "\n");
                self.handle_directive(directive, args).unwrap();
                continue;
            }
        }

        final_tokens
    }

    fn take_until_match(tokens: &mut Vec<DmToken>, pattern: &str) -> Vec<DmToken> {
        return Self::take_until(tokens, |token| token.value() == pattern);
    }

    fn take_until_regex(tokens: &mut Vec<DmToken>, pattern: &Regex) -> Vec<DmToken> {
        return Self::take_until(tokens, |token| pattern.is_match(token.value()));
    }

    fn take_until(tokens: &mut Vec<DmToken>, check: impl Fn(&DmToken) -> bool) -> Vec<DmToken> {
        let mut final_tokens = vec![];

        loop {
            let token = tokens.remove(0);
            if check(&token) {
                return final_tokens;
            }
            final_tokens.push(token);
        }

        error!("take_until pattern never found");
        exit(ERROR_CODE_PATTERN_NOT_FOUND);
    }
}
