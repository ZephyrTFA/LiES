use std::{collections::HashMap, path::PathBuf};

use self::{define_definition::DmDefineDefinition, token_handling::TokenizeState};

pub mod define_definition;
pub mod directive;
pub mod preprocess_core;
mod stddef_defines;
pub mod token_handling;

// #[cfg(test)]
mod tests;

/**
 * The preprocessor is responsible for handling all preprocessor directives.
 * When a file gets preprocessed it is converted into a list of tokens.
 */
pub struct DmPreProcessor {
    defines: HashMap<String, DmDefineDefinition>,
    logical_skip_levels: usize, // if this somehow gets too big, find the nearest bar
    tokenize_in_string: bool,
    tokenize_in_quote: bool,
    pending_includes: Vec<PathBuf>,
    tokenize_state: TokenizeState,
}

impl Default for DmPreProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl DmPreProcessor {
    pub fn new() -> Self {
        Self {
            defines: Self::initial_defines(),
            logical_skip_levels: 0,
            tokenize_in_string: false,
            tokenize_in_quote: false,
            pending_includes: vec![],
            tokenize_state: TokenizeState::default(),
        }
    }

    pub fn add_define(&mut self, define: DmDefineDefinition) {
        self.defines.insert(define.name().to_string(), define);
    }

    pub fn remove_define(&mut self, name: &str) {
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
            Some(define) => define.body().into(),
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
}
