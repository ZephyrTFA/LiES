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
            .iter()
            .filter(|token| !token.processed.is_empty())
            .cloned()
            .collect();

        while last_len != 1 {
            let mut idx = 0;
            while idx < process_chain.len() {
                let token = &process_chain[idx];

                match token.processed.as_str() {
                    "<" => Self::handle_less_than(&mut process_chain, idx),
                    ">" => Self::handle_greater_than(&mut process_chain, idx),
                    "<=" => Self::handle_less_than_or_equal(&mut process_chain, idx),
                    ">=" => Self::handle_greater_than_or_equal(&mut process_chain, idx),
                    "==" => Self::handle_equal(&mut process_chain, idx),
                    "!=" => Self::handle_not_equal(&mut process_chain, idx),
                    "&&" => Self::handle_and_and(&mut process_chain, idx),
                    "||" => Self::handle_or_or(&mut process_chain, idx),
                    "!" => Self::handle_not(&mut process_chain, idx),
                    "(" => Self::handle_open_parenthesis(&mut process_chain, idx),
                    "defined" => self.handle_defined(&mut process_chain, idx),
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

    fn handle_less_than(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(process_chain, idx, left_number < right_number);
        }
    }

    fn handle_greater_than(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(process_chain, idx, left_number > right_number);
        }
    }

    fn handle_less_than_or_equal(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(process_chain, idx, left_number <= right_number);
        }
    }

    fn handle_greater_than_or_equal(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(process_chain, idx, left_number >= right_number);
        }
    }

    fn handle_equal(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(process_chain, idx, left_number == right_number);
        }
    }

    fn handle_not_equal(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(process_chain, idx, left_number != right_number);
        }
    }

    fn handle_and_and(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(
                process_chain,
                idx,
                left_number != 0 && right_number != 0,
            );
        }
    }

    fn handle_or_or(process_chain: &mut Vec<TokenStore>, idx: usize) {
        if let Some((left_number, right_number)) =
            Self::get_left_and_right_as_digits(process_chain, idx)
        {
            Self::replace_comparison_with_result(
                process_chain,
                idx,
                left_number != 0 || right_number != 0,
            );
        }
    }

    fn handle_not(process_chain: &mut Vec<TokenStore>, idx: usize) {
        let next_token = process_chain[idx + 1].processed.as_str().parse::<i32>();

        if next_token.is_err() {
            return;
        }

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

    fn handle_defined(&mut self, process_chain: &mut Vec<TokenStore>, idx: usize) {
        if process_chain[idx + 1].processed != "(" || process_chain[idx + 3].processed != ")" {
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

    fn handle_open_parenthesis(process_chain: &mut Vec<TokenStore>, idx: usize) {
        let next_token = process_chain[idx + 1].processed.as_str().parse::<i32>();

        if process_chain[idx + 2].processed != ")" || next_token.is_err() {
            return;
        }

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

#[cfg(test)]
mod tests {
    use crate::dm_parser::define_definition::DefineDefinition;

    use super::*;

    #[test]
    fn test_handle_less_than() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: "<".to_string(),
                processed: "<".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_less_than(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "1");
    }

    #[test]
    fn test_handle_greater_than() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: ">".to_string(),
                processed: ">".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_greater_than(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "0");
    }

    #[test]
    fn test_handle_less_than_or_equal() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: "<=".to_string(),
                processed: "<=".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_less_than_or_equal(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "1");
    }

    #[test]
    fn test_handle_greater_than_or_equal() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: ">=".to_string(),
                processed: ">=".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_greater_than_or_equal(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "0");
    }

    #[test]
    fn test_handle_equal() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: "==".to_string(),
                processed: "==".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_equal(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "0");
    }

    #[test]
    fn test_handle_not_equal() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: "!=".to_string(),
                processed: "!=".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_not_equal(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "1");
    }

    #[test]
    fn test_handle_and_and() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: "&&".to_string(),
                processed: "&&".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_and_and(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "1");
    }

    #[test]
    fn test_handle_or_or() {
        let mut process_chain = vec![
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: "||".to_string(),
                processed: "||".to_string(),
            },
            TokenStore {
                raw: "3".to_string(),
                processed: "3".to_string(),
            },
        ];
        let idx = 1;

        DmParser::handle_or_or(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "1");
    }

    #[test]
    fn test_handle_not() {
        let mut process_chain = vec![
            TokenStore {
                raw: "!".to_string(),
                processed: "!".to_string(),
            },
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
        ];
        let idx = 0;

        DmParser::handle_not(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "0");
    }

    #[test]
    fn test_handle_open_parenthesis() {
        let mut process_chain = vec![
            TokenStore {
                raw: "(".to_string(),
                processed: "(".to_string(),
            },
            TokenStore {
                raw: "2".to_string(),
                processed: "2".to_string(),
            },
            TokenStore {
                raw: ")".to_string(),
                processed: ")".to_string(),
            },
        ];
        let idx = 0;

        DmParser::handle_open_parenthesis(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "1");
    }

    #[test]
    fn test_handle_open_parenthesis_with_negative_number() {
        let mut process_chain = vec![
            TokenStore {
                raw: "(".to_string(),
                processed: "(".to_string(),
            },
            TokenStore {
                raw: "-2".to_string(),
                processed: "-2".to_string(),
            },
            TokenStore {
                raw: ")".to_string(),
                processed: ")".to_string(),
            },
        ];
        let idx = 0;

        DmParser::handle_open_parenthesis(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "0");
    }

    #[test]
    fn test_handle_defined() {
        let mut process_chain = vec![
            TokenStore {
                raw: "defined".to_string(),
                processed: "defined".to_string(),
            },
            TokenStore {
                raw: "(".to_string(),
                processed: "(".to_string(),
            },
            TokenStore {
                raw: "TEST".to_string(),
                processed: "TEST".to_string(),
            },
            TokenStore {
                raw: ")".to_string(),
                processed: ")".to_string(),
            },
        ];
        let idx = 0;

        let mut parser = DmParser::new("C:/test.dm");
        parser
            .preprocess_state
            .defines
            .insert("TEST".to_string(), DefineDefinition::new_flag("TEST"));

        parser.handle_defined(&mut process_chain, idx);

        assert_eq!(process_chain[0].processed, "1");
    }
}
