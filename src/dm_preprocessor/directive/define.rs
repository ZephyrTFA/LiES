use log::{error, warn};

use crate::{
    dm_preprocessor::{
        define_definition::DmDefineDefinition, token_handling::DmToken, DmPreProcessor,
    },
    util::ParseError,
};

impl DmPreProcessor {
    pub(super) fn handle_define(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.is_empty() {
            error!("`define` directive requires at least one argument");
            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
        }

        let name = args[0].value();
        if args.len() == 1 {
            DmDefineDefinition::new_flag(name).insert_into_map(&mut self.defines);
            return Ok(());
        }

        let define_args = &args[1..];
        if define_args[0].value() == "(" {
            warn!("macros are not implemented yet");
            return Ok(());
        }

        let body: Vec<_> = args.iter().skip(1).cloned().collect();
        DmDefineDefinition::new_basic_replace(
            name,
            &body,
        )
        .insert_into_map(&mut self.defines);

        Ok(())
    }
}
