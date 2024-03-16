use log::{error, warn};

use crate::util::ParseError;

use super::{token_handling::DmToken, DmPreProcessor};

pub mod define;
pub mod error;
pub mod r#if;
pub mod ifdef;
pub mod ifndef;
pub mod include;
pub mod undef;
pub mod warn;

impl DmPreProcessor {
    pub(super) fn handle_directive(
        &mut self,
        directive: &str,
        args: Vec<DmToken>,
    ) -> Result<(), ParseError> {
        let mut effective_args: Vec<DmToken> = vec![];
        for arg in args {
            if arg.value().contains("//") {
                break;
            }
            effective_args.push(arg);
        }

        self.replace_all_defines_possible(&mut effective_args);
        while !effective_args.is_empty() {
            if effective_args
                .first()
                .unwrap()
                .value()
                .chars()
                .all(char::is_whitespace)
            {
                effective_args.remove(0);
            } else if effective_args
                .last()
                .unwrap()
                .value()
                .chars()
                .all(char::is_whitespace)
            {
                effective_args.pop();
            } else {
                break;
            }
        }

        if directive == "else" {
            if !effective_args.is_empty() {
                warn!("`else` directive has arguments that will be ignored");
            };
            if !self.is_skipping() {
                self.increment_logical_skip_level();
            }
            return Ok(());
        }

        if directive == "endif" {
            if !effective_args.is_empty() {
                warn!("`endif` directive has arguments that will be ignored");
            };
            if self.is_skipping() {
                self.decrement_logical_skip_level();
            }
            return Ok(());
        }

        if self.is_skipping() {
            if matches!(directive, "ifdef" | "ifndef" | "if") {
                self.increment_logical_skip_level();
            }
            return Ok(());
        }

        match directive {
            "define" => self.handle_define(&effective_args),
            "error" => self.handle_error(&effective_args),
            "if" => self.handle_if(&effective_args),
            "ifdef" => self.handle_ifdef(&effective_args),
            "ifndef" => self.handle_ifndef(&effective_args),
            "include" => self.handle_include(&effective_args),
            "undef" => self.handle_undef(&effective_args),
            "warn" => self.handle_warn(&effective_args),
            _ => {
                error!(
                    "Unknown directive `{}` with args `{:#?}`",
                    directive, effective_args
                );
                panic!();
            }
        }
    }
}
