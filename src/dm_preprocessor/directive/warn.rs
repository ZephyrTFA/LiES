use log::warn;

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub(super) fn handle_warn(&mut self, args: &[DmToken]) -> Result<(), ()> {
        warn!(
            "Compiler Warning: `{}`",
            args.iter().map(|x| x.value()).collect::<String>()
        );
        Ok(())
    }
}
