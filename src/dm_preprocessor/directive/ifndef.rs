use log::warn;

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub(super) fn handle_ifndef(&mut self, args: &[DmToken]) -> Result<(), String> {
        warn!("`ifdef` directive is not implemented yet");
        Ok(())
    }
}
