use std::process::exit;

use log::{debug, error, warn};

use crate::util::exit_codes::ERROR_CODE_UNKNOWN_DIRECTIVE;

use super::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub(super) fn handle_directive(
        &mut self,
        directive: &str,
        mut args: Vec<DmToken>,
    ) -> Result<(), String> {
        if !args.is_empty() {
            let first_arg: &str = args[0].value();
            if first_arg.chars().all(char::is_whitespace) {
                args.remove(0);
            }
        }

        match directive {
            "include" => self.handle_include(&args),
            "define" => self.handle_define(&args),
            _ => {
                error!("Unknown directive `{}`", directive);
                exit(ERROR_CODE_UNKNOWN_DIRECTIVE);
            }
        }
    }
}
