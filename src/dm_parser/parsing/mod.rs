use std::collections::{HashMap, VecDeque};

use crate::{tokens::dm_token::DmToken, util::dm_file::DmFile};

use super::{
    dm_types::{DmPath, DmProc, DmVar},
    lib::DmParser,
};

#[derive(Default)]
struct ParseState {
    _types: HashMap<String, DmPath>,
    _global_procs: HashMap<String, DmProc>,
    _global_vars: HashMap<String, DmVar>,
}

impl DmParser {
    pub fn parse_tokens(&mut self, tokens: VecDeque<DmToken>, _file: DmFile) {
        let mut state = ParseState::default();
        let _scopes = self.split_into_scopes(&mut state, tokens);
    }

    fn split_into_scopes(
        &mut self,
        _state: &mut ParseState,
        mut _tokens: VecDeque<DmToken>,
    ) -> VecDeque<VecDeque<DmToken>> {
        unimplemented!()
    }
}
