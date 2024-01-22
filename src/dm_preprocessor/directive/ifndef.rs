use log::warn;

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::ParseError,
};

impl DmPreProcessor<'_> {
    pub(super) fn handle_ifndef(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.is_empty() {
            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
        }

        let arg = args[0].value();
        if self.is_defined(arg) {
            self.increment_logical_skip_level();
        }

        Ok(())
    }
}
