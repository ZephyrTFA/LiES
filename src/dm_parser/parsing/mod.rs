#![allow(dead_code)]

use std::collections::VecDeque;

pub mod scope;
pub mod type_path;

use crate::{tokens::dm_token::DmToken, util::ParseError};

use super::lib::DmParser;

impl DmParser {
    pub fn parse_tokens(&mut self, _tokens: VecDeque<DmToken>) -> Result<(), ParseError> {
        unimplemented!()
    }
}
