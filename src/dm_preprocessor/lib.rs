use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use log::{debug, error};

#[cfg(test)]
use once_cell::sync::Lazy;

use crate::{tokens::dm_token::DmToken, util::count_backslashes};

use super::{define_definition::DmDefineDefinition, tokenize_state::TokenizeState};

/**
 * The preprocessor is responsible for handling all preprocessor directives.
 * When a file gets preprocessed it is converted into a list of tokens.
 */
pub struct DmPreProcessor {
    pub defines: HashMap<String, DmDefineDefinition>,
    logical_skip_levels: usize, // if this somehow gets too big, find the nearest bar
    pub pending_includes: Vec<PathBuf>,
    pub tokenize_state: TokenizeState,
    include_order: Vec<PathBuf>,
}

impl Default for DmPreProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl DmPreProcessor {
    pub fn new() -> Self {
        let mut _self = Self {
            defines: HashMap::new(),
            logical_skip_levels: 0,
            pending_includes: vec![],
            tokenize_state: TokenizeState::default(),
            include_order: vec![],
        };
        for define in Self::initial_defines() {
            _self.add_define(define);
        }
        _self
    }

    pub fn add_to_include_order(&mut self, path: &Path) {
        self.include_order.push(PathBuf::from(path));
    }

    pub fn is_included(&self, path: &PathBuf) -> bool {
        self.include_order.contains(path)
    }

    pub fn get_include_order(&self) -> &[PathBuf] {
        &self.include_order
    }

    /// Returns the current file being processed.
    /// This is not guaranteed to be correct as this can be called after the environment has been
    /// parsed
    pub fn get_current_file(&self) -> &PathBuf {
        #[cfg(test)]
        {
            static TEST_FILE: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("test.dm"));
            &TEST_FILE
        }
        #[cfg(not(test))]
        self.include_order
            .last()
            .expect(r"Failed to get the current file")
    }

    pub fn add_define(&mut self, define: DmDefineDefinition) {
        debug!("Adding define `{}`", define.name());
        assert!(define.name() != "1");
        // name cannot start or end with whitespace
        assert!(!define.name().starts_with(char::is_whitespace));
        assert!(!define.name().ends_with(char::is_whitespace));
        // body cannot start or end with whitespace
        if !define.body().is_empty() {
            assert!(!define.body().first().unwrap().value().is_empty());
            assert!(!define.body().last().unwrap().value().is_empty());
        }
        self.defines.insert(define.name().to_string(), define);
    }

    pub fn remove_define(&mut self, name: &str) {
        debug!("Removing define `{}`", name);
        self.defines.remove(name);
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.defines.contains_key(name)
    }

    pub fn get_define(&self, name: &str) -> Option<&DmDefineDefinition> {
        self.defines.get(name)
    }

    pub fn get_base_file_dir(&self) -> PathBuf {
        match self.get_define("BASE_FILE_DIR") {
            Some(define) => {
                assert!(
                    define.body().len() == 1,
                    "BASE_FILE_DIR should only have one token"
                );
                return define.body()[0].value().into();
            }
            None => ".".into(),
        }
    }

    pub fn increment_logical_skip_level(&mut self) {
        self.logical_skip_levels += 1;
    }

    pub fn decrement_logical_skip_level(&mut self) {
        self.logical_skip_levels -= 1;
    }

    pub fn logical_skip_level(&self) -> usize {
        self.logical_skip_levels
    }

    pub fn is_skipping(&self) -> bool {
        self.logical_skip_levels > 0
    }

    pub fn take_pending_includes(&mut self) -> Vec<PathBuf> {
        std::mem::take(&mut self.pending_includes)
    }

    fn do_macro_replacement(
        _macro_definition: &DmDefineDefinition,
        tokens: &mut Vec<DmToken>,
    ) -> Option<DmToken> {
        if tokens.is_empty() || tokens.remove(0).value() != "(" {
            error!("malformed macro call");
            panic!();
        }

        let mut args: Vec<Vec<DmToken>> = vec![vec![]];
        let mut paren_count = 1;
        while !tokens.is_empty() {
            let token = tokens.remove(0);
            let current_args = args.last_mut().unwrap();
            match token.value() {
                ")" if !current_args
                    .last()
                    .is_some_and(|tok| count_backslashes(tok.value()) % 2 == 0) =>
                {
                    paren_count -= 1;
                    if paren_count == 0 {
                        break;
                    }
                }
                "(" if !current_args
                    .last()
                    .is_some_and(|tok| count_backslashes(tok.value()) % 2 == 0) =>
                {
                    paren_count += 1;
                }
                "," if paren_count == 1 => {
                    args.push(vec![]);
                }
                _ => {
                    current_args.push(token);
                }
            }
        }

        #[cfg(not(debug_assertions))]
        warn!("macros are not yet implemented, '{macro_definition:#?}' args {args:#?}");
        None
    }

    /// Prefer do_define_replacement wherever possible to not ignore defines as they get added.
    /// This should only be called in places where you know that no defines will be added.
    /// Such as inside of a macro or preprocessor directive parsing.
    pub fn replace_all_defines_possible(
        &self,
        tokens: &mut Vec<DmToken>,
        in_preprocessoer_directive: bool,
    ) {
        let mut return_tokens = vec![];

        while !tokens.is_empty() {
            let mut token = Some(tokens.remove(0));

            match in_preprocessoer_directive {
                true if token.as_ref().unwrap().value() == "defined" => {
                    return_tokens.push(token.unwrap());
                    return_tokens.push(tokens.remove(0)); // (
                    return_tokens.push(tokens.remove(0)); // identifier
                    return_tokens.push(tokens.remove(0)); // )
                    continue;
                }
                _ => {
                    token = self.do_define_replacement(token.unwrap(), tokens);
                }
            };

            if let Some(token) = token {
                return_tokens.push(token);
            }
        }

        *tokens = return_tokens;
    }

    pub fn do_define_replacement(
        &self,
        token: DmToken,
        next_tokens: &mut Vec<DmToken>,
    ) -> Option<DmToken> {
        let define = self.get_define(token.value());
        if define.is_none() {
            return Some(token);
        }

        let define = define.unwrap();
        if define.is_macro() {
            return Self::do_macro_replacement(define, next_tokens);
        }

        let mut tokens = define.body().to_vec();
        if tokens.is_empty() {
            return None;
        }

        let token = tokens.remove(0);
        next_tokens.splice(0..0, tokens);
        Some(token)
    }
}
