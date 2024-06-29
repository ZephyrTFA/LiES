use std::collections::VecDeque;

use log::{debug, error};

use crate::{
    preprocess::PreprocessState,
    tokenize::token::Token,
    util::parse_error::{IfMalformReason, ParseError, ParseErrorCode},
};

use super::DirectiveResult;

fn must_parse_as_int(value: &str) -> Result<i32, ParseError> {
    value.parse::<i32>().map_err(|_| {
        error!("Failed to parse `{value}` as i32.");
        ParseError::new(ParseErrorCode::MalformedIf(
            IfMalformReason::FailedToParseAsInt,
        ))
    })
}

const CMP_EQ: &str = "==";
const CMP_AND: &str = "&&";
const CMP_OR: &str = "||";
const CMP_NE: &str = "!=";
const CMP_LT: &str = "<";
const CMP_GT: &str = ">";
const CMP_LTE: &str = "<=";
const CMP_GTE: &str = ">=";
const NOT: &str = "!";

const ORDER_OPS: [&str; 10] = [
    "(", NOT, CMP_EQ, CMP_NE, CMP_LT, CMP_GT, CMP_LTE, CMP_GTE, CMP_AND, CMP_OR,
];

impl PreprocessState {
    pub(super) fn handle_directive_if(&mut self, tokens: VecDeque<Token>) -> DirectiveResult {
        let current_run: Vec<Token> = tokens.into_iter().collect();
        let mut current_run: Vec<&str> = current_run
            .iter()
            .filter(|tok| !tok.is_only_spacing())
            .map(|tok| tok.value().as_str())
            .collect();

        macro_rules! expect_token {
            ($idx: expr, $token: literal, $consume: literal) => {
                if $consume {
                    if current_run.remove($idx) != $token {
                        return Err(ParseError::new(ParseErrorCode::UnexpectedEOL));
                    }
                } else if current_run[$idx] != $token {
                    return Err(ParseError::new(ParseErrorCode::UnexpectedEOL));
                }
            };
        }

        while let Some(defined_index) = current_run.iter().position(|tok| *tok == "defined") {
            current_run.remove(defined_index);
            expect_token!(defined_index, "(", true);
            let define_name = current_run.remove(defined_index);
            expect_token!(defined_index, ")", false);

            current_run[defined_index] = if self
                .environment()
                .defines()
                .get_define(define_name)
                .is_some()
            {
                "1"
            } else {
                "0"
            }
        }

        loop {
            debug!("if: {current_run:?}");
            let mut did_something = false;
            for operation in ORDER_OPS {
                if !current_run.contains(&operation) {
                    continue;
                }
                let indexes: Vec<usize> = current_run
                    .iter()
                    .enumerate()
                    .rev()
                    .filter(|(_, str)| **str == operation)
                    .map(|(idx, _)| idx)
                    .collect();

                for index in indexes {
                    macro_rules! must_get_previous {
                        () => {
                            current_run
                                .get(index - 1)
                                .ok_or(ParseError::new(ParseErrorCode::UnexpectedEOL))?
                        };
                    }
                    macro_rules! must_get_next {
                        () => {
                            current_run
                                .get(index + 1)
                                .ok_or(ParseError::new(ParseErrorCode::UnexpectedEOL))?
                        };
                    }
                    macro_rules! skip_if_parenthesis {
                        () => {
                            if *must_get_previous!() == ")" || *must_get_next!() == "(" {
                                continue;
                            }
                        };
                    }
                    macro_rules! get_lhs_and_rhs {
                        () => {
                            (
                                must_parse_as_int(must_get_previous!())?,
                                must_parse_as_int(must_get_next!())?,
                            )
                        };
                    }
                    macro_rules! replace_with_result {
                        ($result: expr) => {
                            current_run.remove(index + 1);
                            current_run.remove(index);
                            current_run[index - 1] = $result;
                            did_something = true;
                            debug!("work({}): {current_run:?}", line!());
                        };
                    }

                    match operation {
                        NOT => {
                            if *must_get_next!() == "(" {
                                continue;
                            }
                            current_run.remove(index);

                            did_something = true;
                            current_run[index] = if must_parse_as_int(current_run[index])? != 0 {
                                "0"
                            } else {
                                "1"
                            };
                            debug!("work({}): {current_run:?}", line!());
                        }
                        "(" => {
                            if current_run.get(index + 2).is_some_and(|tok| *tok == ")") {
                                did_something = true;
                                current_run.remove(index);
                                current_run.remove(index + 1);
                            }
                        }
                        CMP_AND => {
                            skip_if_parenthesis!();
                            let (lhs, rhs) = get_lhs_and_rhs!();
                            replace_with_result!(if lhs == 0 || rhs == 0 { "0" } else { "1" });
                        }
                        CMP_OR => {
                            skip_if_parenthesis!();
                            let (lhs, rhs) = get_lhs_and_rhs!();
                            replace_with_result!(if lhs == 1 || rhs == 1 { "1" } else { "0" });
                        }
                        CMP_GT => {
                            skip_if_parenthesis!();
                            let (lhs, rhs) = get_lhs_and_rhs!();
                            replace_with_result!(if lhs > rhs { "1" } else { "0" });
                        }
                        CMP_LT => {
                            skip_if_parenthesis!();
                            let (lhs, rhs) = get_lhs_and_rhs!();
                            replace_with_result!(if lhs < rhs { "1" } else { "0" });
                        }
                        _ => {
                            error!("unimplemented: {operation}");
                            return Err(ParseError::new(ParseErrorCode::Internal));
                        }
                    }
                }
            }

            if current_run.len() == 1 {
                if must_parse_as_int(current_run[0])? == 0 {
                    self.increment_directive_skip_level()
                }
                return Ok(());
            }

            if !did_something {
                debug!("{current_run:#?}");
                return Err(ParseError::new(ParseErrorCode::MalformedIf(
                    IfMalformReason::ParsingStall,
                )));
            }
        }
    }
}
