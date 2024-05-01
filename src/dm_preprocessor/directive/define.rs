use log::debug;
#[warn(unused_imports)]
use log::{error, warn};

use crate::{
    dm_preprocessor::{
        define_definition::{DmDefineDefinition, MacroParamInfo},
        lib::DmPreProcessor,
    },
    tokens::dm_token::DmToken,
    util::{is_valid_identifier, ParseError},
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

    fn handle_macro(&mut self, _name: &str, args: &[DmToken]) -> Result<(), ParseError> {
        let mut args: Vec<DmToken> = args.to_vec();
        if args.is_empty() {
            return Err(ParseError::ERROR_MACRO_EMPTY_BODY);
        }
        if args.len() < 3 {
            return Err(ParseError::ERROR_MACRO_NOT_ENOUGH_ARGS);
        }

        assert_eq!(args.remove(0).value(), "(");
        let mut arg_names: Vec<String> = vec![];
        while args.first().is_some_and(|x| x.value() != ")") {
            let arg = args.remove(0);
            let arg = arg.value();
            if arg == "..." {
                let last_arg = arg_names.pop();
                arg_names.push(format!("{}...", last_arg.unwrap_or_default()));
                continue;
            }
            if arg == "," || arg == " " {
                debug!("skipping whitespace or comma");
                continue;
            }
            arg_names.push(arg.to_string());
        }
        if !args.first().is_some_and(|x| x.value() == ")") {
            return Err(ParseError::ERROR_MACRO_MALFORMED_ARGUMENTS);
        }

        args.remove(0);
        if args.first().is_some_and(|x| x.value() == " ") {
            args.remove(0);
        }

        // verify all arg names are alphanumeric, except last one which has special behavior
        if arg_names.len() > 1 {
            debug!("checking arg names");
            for arg in arg_names.iter().rev().skip(1).rev() {
                if !is_valid_identifier(arg) {
                    return Err(ParseError::ERROR_MACRO_ARG_NAME_INVALID_CHAR);
                }
            }
        }

        let mut has_ellipsis = false;

        // verify last arg name is alphanumeric, except for trailing ellipsis
        // note that ellipsis are optional
        if !arg_names.is_empty() {
            debug!("checking last arg name");
            let last_arg = arg_names.last().unwrap();
            if last_arg != "..." {
                let mut actual_name: &str = last_arg;
                if last_arg.ends_with("...") {
                    actual_name = &last_arg[..last_arg.len() - 3];
                    has_ellipsis = true;
                }
                if !is_valid_identifier(actual_name) {
                    return Err(ParseError::ERROR_MACRO_ARG_NAME_INVALID_CHAR);
                }
            } else {
                has_ellipsis = true;
            }
        }

        let arg_count = arg_names.len();
        self.add_define(DmDefineDefinition::new_macro(
            _name,
            args,
            MacroParamInfo::new(arg_names, arg_count, has_ellipsis),
        ));
        Ok(())
    }
}
