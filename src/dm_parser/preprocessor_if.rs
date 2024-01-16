use std::io::Result;

use crate::is_verbose;

use super::{token_store::TokenStore, DmParser};

impl DmParser<'_> {
    fn get_left_and_right_as_digits(
        process_chain: &[TokenStore],
        idx: usize,
    ) -> Option<(i32, i32)> {
        if idx == 0 || idx == process_chain.len() - 1 {
            return None;
        }

        let left_number = process_chain[idx - 1].processed.parse::<i32>();
        let right_number = process_chain[idx + 1].processed.parse::<i32>();
        if left_number.is_err() || right_number.is_err() {
            return None;
        }

        Some((left_number.unwrap(), right_number.unwrap()))
    }

    fn replace_comparison_with_result(
        process_chain: &mut Vec<TokenStore>,
        idx: usize,
        result: bool,
    ) {
        process_chain.remove(idx + 1);
        process_chain.remove(idx);
        process_chain.remove(idx - 1);
        let result = if result { "1" } else { "0" };
        process_chain.insert(
            idx - 1,
            TokenStore {
                raw: result.to_string(),
                processed: result.to_string(),
            },
        );
    }

    pub fn parse_preprocessor_if(&mut self, tokens: &[TokenStore]) -> Result<()> {
        let mut last_len = tokens.len();
        let mut process_chain: Vec<TokenStore> = tokens
            .to_vec()
            .into_iter()
            .filter(|token| !token.processed.is_empty())
            .collect();

        while last_len != 1 {
            let mut idx = 0;
            while idx < process_chain.len() {
                let token = &process_chain[idx];

                match token.processed.as_str() {
                    "<" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number < right_number,
                            );
                        }
                    }
                    ">" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number > right_number,
                            );
                        }
                    }
                    "<=" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number <= right_number,
                            );
                        }
                    }
                    ">=" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number >= right_number,
                            );
                        }
                    }
                    "==" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number == right_number,
                            );
                        }
                    }
                    "!=" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number != right_number,
                            );
                        }
                    }
                    "&&" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number != 0 && right_number != 0,
                            );
                        }
                    }
                    "||" => {
                        if let Some((left_number, right_number)) =
                            Self::get_left_and_right_as_digits(&process_chain, idx)
                        {
                            Self::replace_comparison_with_result(
                                &mut process_chain,
                                idx,
                                left_number != 0 || right_number != 0,
                            );
                        }
                    }
                    "defined" => {
                        if process_chain[idx + 1].processed != "("
                            || process_chain[idx + 3].processed != ")"
                        {
                            panic!("failed to parse if directive: {:?}", process_chain);
                        }
                        let token_name = &process_chain[idx + 2].processed;
                        let result = self.preprocess_state.defines.contains_key(token_name);
                        process_chain.remove(idx + 3);
                        process_chain.remove(idx + 2);
                        process_chain.remove(idx + 1);
                        process_chain.remove(idx);
                        let result = if result { "1" } else { "0" };
                        process_chain.insert(
                            idx,
                            TokenStore {
                                raw: result.to_string(),
                                processed: result.to_string(),
                            },
                        );
                    }
                    "!" => {
                        let next_token = process_chain[idx + 1].processed.as_str().parse::<i32>();
                        if next_token.is_ok() {
                            let next_token = next_token.unwrap();
                            process_chain.remove(idx + 1);
                            process_chain.remove(idx);
                            let result = if next_token <= 0 { "1" } else { "0" };
                            process_chain.insert(
                                idx,
                                TokenStore {
                                    raw: result.to_string(),
                                    processed: result.to_string(),
                                },
                            );
                        }
                    }
                    "(" => {
                        let next_token = process_chain[idx + 1].processed.as_str().parse::<i32>();
                        if process_chain[idx + 2].processed == ")" && next_token.is_ok() {
                            let next_token = next_token.unwrap();
                            process_chain.remove(idx + 2);
                            process_chain.remove(idx + 1);
                            process_chain.remove(idx);
                            let result = if next_token > 0 { "1" } else { "0" };
                            process_chain.insert(
                                idx,
                                TokenStore {
                                    raw: result.to_string(),
                                    processed: result.to_string(),
                                },
                            );
                        }
                    }
                    _ => {}
                }
                idx += 1;
            }

            let new_len = process_chain.len();
            if last_len == new_len {
                panic!("failed to parse if directive: {:?}", process_chain);
            }
            last_len = new_len;
        }

        if process_chain[0].processed != "0" && process_chain[0].processed != "1" {
            panic!("failed to parse if directive: {:?}", process_chain);
        }
        let final_result = process_chain[0].processed == "1";
        if is_verbose() {
            println!("if directive result: {}", final_result);
        }

        if !final_result {
            self.preprocess_state.skip_levels += 1;
        }

        Ok(())
    }
}
