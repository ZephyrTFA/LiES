use log::{debug, error, trace};
use once_cell::sync::Lazy;

use crate::{
    dm_preprocessor::{token::dm_token::DmToken, DmPreProcessor},
    util::ParseError,
};

impl DmPreProcessor {
    // parentheses are last because we only resolve them when all the ops inside of them are resolved
    const ORDER_OF_OPS: [&'static str; 9] = ["!", "!=", ">", ">=", "<", "<=", "&&", "||", "("];

    fn define_replacement(&self, name: &str) -> String {
        if let Some(define) = self.get_define(name) {
            return define.body().to_owned();
        }
        name.to_owned()
    }

    pub(super) fn handle_if(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        let mut args: Vec<String> = args
            .iter()
            .map(|x| x.value().trim())
            .filter(|x: &&str| !x.is_empty())
            .map(|x: &str| self.define_replacement(x))
            .collect();

        if args.is_empty() {
            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
        }

        static DEFINED: Lazy<String> = Lazy::new(|| "defined".to_string());
        while args.contains(&DEFINED) {
            let index = args.iter().position(|x| x == DEFINED.as_str()).unwrap();
            let left_paren = args.get(index + 1).map(|x| x == "(").unwrap_or(false);
            let right_paren = args.get(index + 3).map(|x| x == ")").unwrap_or(false);
            if !left_paren || !right_paren {
                error!("Malformed defined() macro. {args:#?}");
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }
            let name = args.remove(index + 2);
            let defined = self.get_define(&name).is_some();
            args[index] = if defined { "1" } else { "0" }.to_owned();
            args.remove(index + 1);
            args.remove(index + 1);
            trace!("Defined {} as {}", name, defined);
        }

        'top: loop {
            debug!("if: {:#?}", args);
            for wanted_op in Self::ORDER_OF_OPS {
                trace!("processing if: {:#?}", args);
                let mut op_indexes = args
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| if *x == wanted_op { Some(i) } else { None })
                    .collect::<Vec<usize>>();
                op_indexes.reverse();
                trace!("Found {} instances of {}", op_indexes.len(), wanted_op);
                for op_index in op_indexes {
                    match wanted_op {
                        "||" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs > 0 || rhs > 0 {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        "&&" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs > 0 && rhs > 0 {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        "!" => {
                            if let Some(Ok(rhs)) = args.get(op_index + 1).map(|x| x.parse::<i32>())
                            {
                                args[op_index] = if rhs > 0 {
                                    "0".to_owned()
                                } else {
                                    "1".to_owned()
                                };
                                args.remove(op_index + 1);
                                continue 'top;
                            }
                        }
                        "==" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs == rhs {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        "!=" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs != rhs {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        ">" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs > rhs {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        ">=" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs >= rhs {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        "<" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs < rhs {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        "<=" => {
                            if let Some(Ok(lhs)) = args.get(op_index - 1).map(|x| x.parse::<i32>())
                            {
                                if let Some(Ok(rhs)) =
                                    args.get(op_index + 1).map(|x| x.parse::<i32>())
                                {
                                    args[op_index - 1] = if lhs <= rhs {
                                        "1".to_owned()
                                    } else {
                                        "0".to_owned()
                                    };
                                    args.remove(op_index);
                                    args.remove(op_index);
                                    continue 'top;
                                }
                            }
                        }
                        "(" => {
                            // check for ( X )
                            if args.get(op_index + 2).is_some_and(|x| x == ")")
                                && !args.get(op_index + 1).is_some_and(|x| x == "(")
                            {
                                args.remove(op_index + 2);
                                args.remove(op_index);
                                continue 'top;
                            }
                        }
                        _ => {
                            error!("Unknown operator {}", wanted_op);
                            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                        }
                    }
                }
            }

            if args.len() > 1 {
                error!("Malformed if statement: {:#?}", args);
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }
            if let Ok(result) = args[0].parse::<i32>() {
                if result <= 0 {
                    self.increment_logical_skip_level();
                }
                return Ok(());
            } else {
                error!("Malformed if statement");
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }
        }
    }
}
