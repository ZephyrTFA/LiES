use std::collections::VecDeque;

use log::{debug, error, warn};

use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken, util::ParseError};

impl DmPreProcessor {
    pub fn handle_directive(
        &mut self,
        directive: &str,
        args: &[DmToken],
    ) -> Result<(), ParseError> {
        debug!("Handling directive `{directive}` with args `{args:#?}`");

        // explicitly handle define preprocessing here before we perform define replacement
        match directive {
            "ifdef" => return self.handle_ifdef(args),
            "ifndef" => return self.handle_ifndef(args),
            "define" => return self.handle_define(args),
            "undef" => return self.handle_undef(args),
            _ => {}
        }

        let mut effective_args: VecDeque<DmToken> = VecDeque::new();
        effective_args.reserve_exact(args.len());
        for arg in args {
            if arg.value().contains("//") {
                break;
            }
            effective_args.push_back(arg.clone());
        }

        self.replace_all_defines_possible(&mut effective_args, true)?;
        while !effective_args.is_empty() {
            if effective_args
                .front()
                .unwrap()
                .value()
                .chars()
                .all(char::is_whitespace)
            {
                effective_args.pop_front().unwrap();
            } else if effective_args
                .back()
                .unwrap()
                .value()
                .chars()
                .all(char::is_whitespace)
            {
                effective_args.pop_back().unwrap();
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

        effective_args.make_contiguous();
        let directive_args = effective_args.as_slices().0;
        match directive {
            "error" => self.handle_error(directive_args),
            "if" => self.handle_if(directive_args),
            "include" => self.handle_include(directive_args),
            "warn" => self.handle_warn(directive_args),
            _ => {
                error!(
                    "Unhandled directive `{}` with args `{:#?}`",
                    directive, directive_args
                );
                panic!();
            }
        }
    }
}
