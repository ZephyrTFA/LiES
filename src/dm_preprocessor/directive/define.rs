use log::warn;

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub(super) fn handle_define(&mut self, args: &[DmToken]) -> Result<(), String> {
        warn!("`define` directive is not implemented yet");
        Ok(())
    }
}
