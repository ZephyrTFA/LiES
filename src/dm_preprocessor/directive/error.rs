use log::error;

use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken, util::ParseError};

impl DmPreProcessor {
    pub(super) fn handle_error(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        error!(
            "Compiler Error: `{}`",
            args.iter().map(|x| x.value()).collect::<String>()
        );
        Err(ParseError::ERROR_FORCED)
    }
}
