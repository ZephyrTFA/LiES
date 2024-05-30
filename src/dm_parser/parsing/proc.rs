use std::collections::VecDeque;

use crate::{
    dm_parser::{
        consume_spacing,
        dm_types::{DmVar, DmVarDefaultValue},
        lib::DmParser,
        parsing::type_path::DmTypePath,
    },
    tokens::dm_token::DmToken,
    util::ParseError,
};

use super::scope::Scope;

pub struct DmProcRaw {
    name: String,
    params: Vec<DmVar>,
    settings: Vec<DmVar>,
    body: Vec<DmToken>,
}

fn print_next_7(tokens: &VecDeque<DmToken>) {
    let next_7 = tokens
        .iter()
        .take(7)
        .map(|tok| tok.value())
        .collect::<Vec<&str>>();
    println!("N7: {next_7:?}");
}

impl DmParser {
    pub(super) fn consume_proc(
        &mut self,
        tokens: &mut VecDeque<DmToken>,
        scope: &mut Scope,
    ) -> Result<DmProcRaw, ParseError> {
        if tokens.is_empty() {
            return Err(ParseError::UNEXPECTED_EOL);
        }

        let name = scope.type_path().unwrap().last();
        println!("proc name: {}", name);

        // grab params
        consume_spacing(tokens);
        if tokens.pop_front().unwrap().value() != "(" {
            return Err(ParseError::UNEXPECTED_EOL);
        }

        let mut params = Vec::new();
        while let Some(token) = tokens.pop_front() {
            let next = Self::consume_type_path(token, tokens)?;
            consume_spacing(tokens);
            let mut default_value = DmVarDefaultValue::Null;
            if tokens.front().unwrap().value() == "=" {
                tokens.pop_front();
                consume_spacing(tokens);
                default_value = DmVarDefaultValue::from_tokens(tokens)?;
                consume_spacing(tokens);
            }

            let mut parts = next.parts();
            if parts.first().is_some_and(|s| *s == "var") {
                parts.remove(0);
            }
            let param_name = parts.pop().unwrap();
            let implied_type = if !parts.is_empty() {
                Some(DmTypePath::from(parts.as_slice()))
            } else {
                None
            };

            println!("param: {param_name} {implied_type:?} {default_value:?}");
            params.push(DmVar::new(
                param_name.to_string(),
                implied_type,
                default_value,
            ));

            let front = tokens.front();
            if front.is_none() {
                return Err(ParseError::UNEXPECTED_EOL);
            }
            let front = front.unwrap();
            match front.value() {
                "," => {
                    tokens.pop_front();
                    consume_spacing(tokens);
                }
                ")" => break,
                _ => return Err(ParseError::EXPECTED_DIFFERENT_TOKEN),
            }
        }

        if !tokens.pop_front().is_some_and(|tok| tok.value() == ")") {
            return Err(ParseError::EXPECTED_DIFFERENT_TOKEN);
        }
        consume_spacing(tokens);

        unimplemented!()
    }
}
