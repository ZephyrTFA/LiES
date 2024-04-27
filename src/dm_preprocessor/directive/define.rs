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
            self.add_define(DmDefineDefinition::new_flag(name));
            return Ok(());
        }

        let define_args = &args[1..];
        if define_args[0].value() == "(" {
            return self.handle_macro(name, define_args);
        }

        let body: Vec<_> = args.iter().skip(1).cloned().collect();
        self.add_define(DmDefineDefinition::new_basic_replace(name, &body));

        Ok(())
    }

    fn handle_macro(&mut self, _name: &str, _args: &[DmToken]) -> Result<(), ParseError> {
        #[cfg(not(debug_assertions))]
        warn!("macros are not implemented yet");
        Ok(())
    }
}
