use log::warn;

use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken, util::ParseError};

impl DmPreProcessor {
    pub(super) fn handle_ifdef(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.is_empty() {
            warn!("`ifdef` directive requires at least one argument");
            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
        }

        let define_name = args[0].value();
        if !self.defines.contains_key(define_name) {
            self.increment_logical_skip_level();
        }
        Ok(())
    }
}
