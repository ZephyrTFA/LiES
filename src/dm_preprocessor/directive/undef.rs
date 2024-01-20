use log::warn;

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub fn handle_undef(&mut self, args: &[DmToken]) -> Result<(), String> {
        warn!("`undef` directive is not implemented yet");
        Ok(())
    }
}
