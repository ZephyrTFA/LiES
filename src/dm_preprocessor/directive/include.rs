use log::debug;

use crate::dm_preprocessor::{token_handling::DmToken, DmPreProcessor};

impl DmPreProcessor {
    pub fn handle_include(&mut self, args: &[DmToken]) -> Result<(), String> {
        if args.len() != 3 {
            return Err(format!(
                "Invalid number of arguments for `include` (expected 3, got {})",
                args.len()
            ));
        }
        if args[0].value() != "\"" || args[2].value() != "\"" {
            return Err("Invalid argument format for `include`".to_owned());
        }
        debug!("include: `{}`", args[1].value());
        self.pending_includes.push(args[1].value().into());
        Ok(())
    }
}
