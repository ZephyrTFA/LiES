use std::collections::{HashMap, VecDeque};

use crate::{tokens::dm_token::DmToken, util::ParseError};

use super::{consume_spacing, lib::DmParser, parsing::type_path::DmTypePath};

pub struct DmPath {
    _parent_path: Option<String>,
    _vars: HashMap<String, DmVar>,
    // _procs: HashMap<String, DmProc>,
}

pub struct DmVar {
    name: String,
    implied_type: Option<DmTypePath>,
    default_value: DmVarDefaultValue,
}

impl DmVar {
    pub fn new(
        name: String,
        implied_type: Option<DmTypePath>,
        default_value: DmVarDefaultValue,
    ) -> Self {
        Self {
            name,
            implied_type,
            default_value,
        }
    }
}

#[derive(Debug)]
pub enum DmVarDefaultValue {
    Float(f64),
    String(String),
    Type(String),
    Datum(String),
    NewImplied,
    New(DmTypePath),
    Null,
}

impl DmVarDefaultValue {
    pub fn from_tokens(tokens: &mut VecDeque<DmToken>) -> Result<Self, ParseError> {
        if tokens.is_empty() {
            return Ok(Self::Null);
        }

        let front = tokens.pop_front().unwrap();
        consume_spacing(tokens);

        if front.value() == "new" {
            if tokens.is_empty() {
                return Ok(Self::NewImplied);
            }
            if tokens.front().unwrap().value() != "/" {
                return Ok(Self::NewImplied); // be forgiving here
            }
            return Ok(Self::New(DmParser::consume_type_path(
                tokens.pop_front().unwrap(),
                tokens,
            )?));
        }

        Err(ParseError::EXPECTED_DIFFERENT_TOKEN)
    }
}
