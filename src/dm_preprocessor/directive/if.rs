use log::{debug, error, trace, warn};

use crate::{
    dm_preprocessor::{token_handling::DmToken, DmPreProcessor},
    util::ParseError,
};

impl DmPreProcessor {
    pub(super) fn handle_if(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        trace!("handle_if: {:#?}", args);
        let mut args: Vec<&str> = args
            .iter()
            .map(|x| x.value().trim())
            .filter(|x: &&str| !x.is_empty())
            .collect();

        if args.is_empty() {
            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
        }

        'start: loop {
            let start_length = args.len();
            let mut define_replaced = false;
            for idx in 0..args.len() {
                let arg = args[idx];
                match arg {
                    "(" => {
                        if args
                            .get(idx + 2)
                            .is_some_and(|&closing_parenthesis| closing_parenthesis == ")")
                        {
                            let next: Option<Result<i32, _>> = args
                                .get(idx + 1)
                                .filter(|&&target| target != "(")
                                .map(|target| target.parse::<i32>());
                            if let Some(Ok(_)) = next {
                                args.remove(idx + 2);
                                args.remove(idx);
                                continue 'start;
                            }
                        }
                    }
                    "defined" => {}
                    "!" => {}
                    "!=" => {}
                    "&&" => {}
                    "||" => {}
                    ">" => {}
                    ">=" => {}
                    "<" => {}
                    "<=" => {}
                    _ => {
                        if let Some(define) = self.get_define(arg) {
                            if !define.body().is_empty() {
                                args[idx] = define.body();
                                define_replaced = true;
                            }
                        }
                        continue;
                    }
                }
            }

            if !define_replaced && args.len() == start_length {
                error!("Unable to parse if statement: remaining args: {:#?}", args);
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }
        }

        Ok(())
    }
}
