use std::process::exit;

use log::{error, warn};

use crate::util::{exit_codes::ERROR_CODE_UNKNOWN_DIRECTIVE, ParseError};

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
        mut args: Vec<DmToken>,
    ) -> Result<(), ParseError> {
        if directive == "else" {
            if !args.is_empty() {
                warn!("`else` directive has arguments that will be ignored");
            };
            if !self.is_skipping() {
                self.increment_logical_skip_level();
            }
            return Ok(());
        }

        if directive == "endif" {
            if !args.is_empty() {
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
            "define" => self.handle_define(&args),
            "error" => self.handle_error(&args),
            "if" => self.handle_if(&args),
            "ifdef" => self.handle_ifdef(&args),
            "ifndef" => self.handle_ifndef(&args),
            "include" => self.handle_include(&args),
            "undef" => self.handle_undef(&args),
            "warn" => self.handle_warn(&args),
            _ => {
                error!("Unknown directive `{}` with args `{:#?}`", directive, args);
                exit(ERROR_CODE_UNKNOWN_DIRECTIVE);
            }
        }
    }
}
