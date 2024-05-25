use std::collections::VecDeque;

use log::{error, trace};

use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken, util::ParseError};

impl DmPreProcessor {
    // parentheses are last because we only resolve them when all the ops inside of them are resolved
    const ORDER_OF_OPS: [&'static str; 9] = ["!", "!=", ">", ">=", "<", "<=", "&&", "||", "("];
    const DEFINED: &'static str = "defined";

    pub(super) fn handle_if(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.is_empty() {
            error!("attempted to parse `if` directive with no arguments");
            panic!();
        }

        trace!("`if` directive with args: {:#?}", args);
        let mut current_run = args
            .iter()
            .filter(|x| !x.value().trim().is_empty())
            .cloned()
            .collect::<Vec<_>>();

        // handle all defined() macros
        let mut defined_index = current_run.iter().position(|x| x.value() == Self::DEFINED);
        while defined_index.is_some() {
            let index = defined_index.unwrap();
            if index + 3 >= current_run.len() {
                error!("Malformed preprocessor if directive");
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }

            if current_run[index + 1].value() != "(" || current_run[index + 3].value() != ")" {
                error!("Malformed preprocessor if directive");
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }

            let defined_name = current_run[index + 2].value();
            let is_defined = self.defines.contains_key(defined_name);
            trace!("defined({}) = {}", defined_name, is_defined);
            current_run.remove(index + 3);
            current_run.remove(index + 2);
            current_run.remove(index + 1);
            current_run[index] = DmToken::new(if is_defined {
                "1".to_string()
            } else {
                "0".to_string()
            });

            defined_index = current_run.iter().position(|x| x.value() == Self::DEFINED);
        }

        'top_level: loop {
            let mut did_something = false;
            for op in Self::ORDER_OF_OPS {
                let mut token_indexes = VecDeque::new();
                for (i, token) in current_run.iter().enumerate() {
                    if token.value() == op {
                        token_indexes.push_front(i);
                    }
                }

                for token_index in token_indexes {
                    trace!("if: {:#?}", current_run);
                    let token = current_run[token_index].value();
                    match token {
                        "!" => {
                            if token_index + 1 >= current_run.len() {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let next_token = current_run[token_index + 1].value();
                            if let Ok(value) = next_token.parse::<i32>() {
                                current_run.remove(token_index);
                                current_run[token_index] = DmToken::new(if value == 0 {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                });
                                did_something = true;
                            }
                        }
                        "==" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] = DmToken::new(if left == right {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                });
                                did_something = true;
                            }
                        }
                        "!=" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] = DmToken::new(if left != right {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                });
                                did_something = true;
                            }
                        }
                        ">" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] = DmToken::new(if left > right {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                });
                                did_something = true;
                            }
                        }
                        ">=" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] = DmToken::new(if left >= right {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                });
                                did_something = true;
                            }
                        }
                        "<" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] = DmToken::new(if left < right {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                });
                                did_something = true;
                            }
                        }
                        "<=" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] = DmToken::new(if left <= right {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                });
                                did_something = true;
                            }
                        }
                        "&&" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] =
                                    DmToken::new(if left != 0 && right != 0 {
                                        "1".to_string()
                                    } else {
                                        "0".to_string()
                                    });
                                did_something = true;
                            }
                        }
                        "||" => {
                            if token_index + 1 >= current_run.len() || token_index < 1 {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }

                            let left = current_run[token_index - 1].value();
                            let right = current_run[token_index + 1].value();
                            if let (Ok(left), Ok(right)) =
                                (left.parse::<i32>(), right.parse::<i32>())
                            {
                                current_run.remove(token_index);
                                current_run.remove(token_index);
                                current_run[token_index - 1] =
                                    DmToken::new(if left != 0 || right != 0 {
                                        "1".to_string()
                                    } else {
                                        "0".to_string()
                                    });
                                did_something = true;
                            }
                        }
                        "(" => {
                            if token_index + 3 >= current_run.len() {
                                error!("Malformed preprocessor if directive: {:#?}", current_run);
                                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                            }
                            // check index + 1 isn't a ( or )
                            let next_token = current_run[token_index + 1].value();
                            match next_token {
                                ")" => {
                                    error!(
                                        "Malformed preprocessor if directive: {:#?}",
                                        current_run
                                    );
                                    return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                                }
                                "(" => {
                                    continue;
                                }
                                _ => {}
                            }

                            let end_token = current_run[token_index + 2].value();
                            if end_token != ")" {
                                continue;
                            }

                            current_run.remove(token_index);
                            current_run.remove(token_index + 1);
                            did_something = true;
                        }
                        _ => {
                            error!("unhandled operator: {}", token);
                            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
                        }
                    }
                }

                if did_something {
                    continue 'top_level;
                }
            }

            if current_run.len() != 1 {
                error!("Malformed preprocessor if directive: {:#?}", current_run);
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }

            let result = current_run[0].value().parse::<i32>();
            if result.is_err() {
                error!("Malformed preprocessor if directive: {:#?}", current_run);
                return Err(ParseError::ERROR_DIRECTIVE_PARSE);
            }

            if result.unwrap() == 0 {
                trace!("if: FALSE");
                self.increment_logical_skip_level();
            }
            trace!("if: TRUE");
            return Ok(());
        }
    }
}
