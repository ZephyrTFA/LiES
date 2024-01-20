use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub fn handle_define(&mut self, args: &[DmToken]) -> Result<(), String> {
        Ok(())
    }
}
