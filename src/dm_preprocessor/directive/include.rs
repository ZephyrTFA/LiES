use std::process::exit;

use log::{debug, error, trace};

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::{exit_codes::ERROR_CODE_INVALID_INCLUDE_FORMAT, ParseError},
};

impl DmPreProcessor<'_> {
    pub(super) fn handle_include(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.len() != 3 {
            error!("Invalid argument format for `include`");
            exit(ERROR_CODE_INVALID_INCLUDE_FORMAT);
        }

        if args[0].value() != "\"" || args[2].value() != "\"" {
            error!("Invalid argument format for `include`");
            exit(ERROR_CODE_INVALID_INCLUDE_FORMAT);
        }
        trace!("include: `{}`", args[1].value());
        self.pending_includes.push(args[1].value().into());
        Ok(())
    }
}
