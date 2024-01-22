use log::warn;

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::ParseError,
};

impl DmPreProcessor<'_> {
    pub fn handle_undef(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.len() != 1 {
            warn!("`undef` requires one argument");
            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
        }
        self.defines.remove(args[0].value());
        Ok(())
    }
}
