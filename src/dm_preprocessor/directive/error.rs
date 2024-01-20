use log::{error, warn};

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub(super) fn handle_error(&mut self, args: &[DmToken]) -> Result<(), ()> {
        error!(
            "Compiler Error: `{}`",
            args.iter().map(|x| x.value()).collect::<String>()
        );
        Err(())
    }
}
