use log::warn;

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::ParseError,
};

impl DmPreProcessor {
    pub(super) fn handle_if(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        warn!("`if` directive is not implemented yet");
        Ok(())
    }
}
