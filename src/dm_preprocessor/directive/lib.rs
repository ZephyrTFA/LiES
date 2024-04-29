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

        let mut effective_args: Vec<DmToken> = vec![];
        let args: Vec<DmToken> = args.into();

        for arg in args {
            if arg.value().contains("//") {
                break;
            }
            effective_args.push(arg);
        }

        self.replace_all_defines_possible(&mut effective_args, true);
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
            "error" => self.handle_error(&effective_args),
            "if" => self.handle_if(&effective_args),
            "include" => self.handle_include(&effective_args),
            "warn" => self.handle_warn(&effective_args),
            _ => {
                error!(
                    "Unhandled directive `{}` with args `{:#?}`",
                    directive, effective_args
                );
                panic!();
            }
        }
    }
}
