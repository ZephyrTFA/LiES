use log::warn;

use crate::{
    dm_preprocessor::{token::dm_token::DmToken, DmPreProcessor},
    util::ParseError,
};

impl DmPreProcessor {
    pub(super) fn handle_warn(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        warn!(
            "Compiler Warning: `{}`",
            args.iter().map(|x| x.value()).collect::<String>()
        );
        Ok(())
    }
}
