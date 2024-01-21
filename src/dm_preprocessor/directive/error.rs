use log::{error, warn};

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::ParseError,
};

impl DmPreProcessor {
    pub(super) fn handle_error(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        error!(
            "Compiler Error: `{}`",
            args.iter().map(|x| x.value()).collect::<String>()
        );
        Err(ParseError::ERROR_FORCED)
    }
}
