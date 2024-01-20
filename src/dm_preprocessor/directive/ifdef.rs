use log::warn;

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub(super) fn handle_ifdef(&mut self, args: &[DmToken]) -> Result<(), String> {
        let define_name = args[0].value();
        if !self.defines.contains_key(define_name) {
            self.increment_logical_skip_level();
        }
        Ok(())
    }
}
