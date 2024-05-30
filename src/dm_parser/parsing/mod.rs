#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

pub mod proc;
pub mod scope;
pub mod type_path;

use crate::{tokens::dm_token::DmToken, util::ParseError};

use super::{
    dm_types::{DmPath, DmVar},
    lib::DmParser,
};

#[derive(Default)]
pub struct ObjectTree {
    types: HashMap<String, DmPath>,
    // global_procs: HashMap<String, DmProc>,
    global_vars: HashMap<String, DmVar>,
}

#[derive(Default)]
struct Scope {
    type_path: String,
    tokens: VecDeque<DmToken>,
}

impl DmParser {
    pub fn parse_tokens(&mut self, tokens: VecDeque<DmToken>) -> Result<(), ParseError> {
        let _scopes = self.parse_scopes(tokens)?;
        Ok(())
    }
}
