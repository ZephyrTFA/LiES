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

        let name = args[0].value().to_owned();
        if args.len() == 1 {
            self.defines
                .insert(name.clone(), DmDefineDefinition::new_flag(name));
            return Ok(());
        }

        let define_args = &args[1..];
        if define_args[0].value() == "(" {
            warn!("macros are not implemented yet");
            return Ok(());
        }

        let body = define_args
            .iter()
            .map(|token| token.value())
            .collect::<Vec<_>>()
            .concat()
            .trim()
            .to_owned();
        self.defines.insert(
            name.clone(),
            DmDefineDefinition::new_basic_replace(name, body),
        );

        Ok(())
    }
}
