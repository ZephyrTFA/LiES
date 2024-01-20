use log::warn;

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub fn handle_undef(&mut self, args: &[DmToken]) -> Result<(), ()> {
        if args.len() != 1 {
            warn!("`undef` requires one argument");
            return Err(());
        }
        self.defines.remove(args[0].value());
        Ok(())
    }
}
