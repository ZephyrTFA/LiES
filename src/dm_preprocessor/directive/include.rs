use std::process::exit;

use log::{debug, error, trace};

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::{exit_codes::ERROR_CODE_INVALID_INCLUDE_FORMAT, ParseError},
};

impl DmPreProcessor {
    pub(super) fn handle_include(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.len() != 3 {
            error!("Invalid argument format for `include`");
            panic!();
        }

        if args[0].value() != "\"" || args[2].value() != "\"" {
            error!("Invalid argument format for `include`");
            panic!();
        }
        trace!("include: `{}`", args[1].value());
        self.pending_includes.push(args[1].value().into());
        Ok(())
    }
}
