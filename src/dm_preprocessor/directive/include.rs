

use log::{error, trace};

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::{ParseError},
};

impl DmPreProcessor {
    pub(super) fn handle_include(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.len() != 3 || args[0].value() != "\"" || args[2].value() != "\"" {
            error!("Invalid include format: {args:#?}");
            panic!();
        }
        trace!("include: `{}`", args[1].value());
        self.pending_includes.push(args[1].value().into());
        Ok(())
    }
}
