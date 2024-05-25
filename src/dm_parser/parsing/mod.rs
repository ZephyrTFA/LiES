#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

mod scope;

use crate::{
    tokens::dm_token::DmToken,
    util::{dm_file::DmFile, ParseError},
};

use super::{
    dm_types::{DmPath, DmProc, DmVar},
    lib::DmParser,
};

#[derive(Default)]
pub struct ObjectTree {
    types: HashMap<String, DmPath>,
    global_procs: HashMap<String, DmProc>,
    global_vars: HashMap<String, DmVar>,
}

#[derive(Default)]
struct Scope {
    type_path: String,
    tokens: VecDeque<DmToken>,
}

impl DmParser {
    pub fn parse_tokens(
        &mut self,
        tokens: VecDeque<DmToken>,
        _file: &DmFile,
    ) -> Result<ObjectTree, ParseError> {
        self.generate_object_tree(tokens)
    }

    fn generate_object_tree(
        &mut self,
        _tokens: VecDeque<DmToken>,
    ) -> Result<ObjectTree, ParseError> {
        // let scopes = self.parse_scopes(tokens);
        Ok(ObjectTree::default())
    }
}
