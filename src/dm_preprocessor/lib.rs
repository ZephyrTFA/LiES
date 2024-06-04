use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
};

use log::{debug, error};

#[cfg(test)]
use once_cell::sync::Lazy;

use crate::{tokens::dm_token::DmToken, util::ParseError};

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
        debug!("skip level: {}", self.logical_skip_levels);
    }

    pub fn decrement_logical_skip_level(&mut self) {
        self.logical_skip_levels -= 1;
        debug!("skip level: {}", self.logical_skip_levels);
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
        macro_definition: &DmDefineDefinition,
        tokens: &mut VecDeque<DmToken>,
    ) -> Result<Option<DmToken>, ParseError> {
        if !tokens.pop_front().is_some_and(|tok| tok.value() == "(") {
            return Err(ParseError::EXPECTED_DIFFERENT_TOKEN);
        }

        let param_info = macro_definition.macro_param_info();
        let mut raw_args = vec![];
        let mut arg_split_points = vec![];
        let mut paren_count = 1; // account for the one we popped out
        while !tokens.is_empty() {
            let token = tokens.pop_front();
            if token.is_none() {
                return Err(ParseError::ERROR_MACRO_MALFORMED_CALL);
            }
            let token = token.unwrap();
            if token.value() == ")" && !token.is_in_string() {
                paren_count -= 1;
                if paren_count == 0 {
                    break;
                }
            } else if token.value() == "(" && !token.is_in_string() {
                paren_count += 1;
            } else if token.value() == "," && paren_count == 1 && !token.is_in_string() {
                arg_split_points.push(raw_args.len());
                continue;
            } else if token.value() == " " && paren_count == 1 && !token.is_in_string() {
                continue;
            }
            raw_args.push(token);
        }

        let mut args = vec![];
        for split_point in arg_split_points.iter().rev() {
            let arg = raw_args.split_off(*split_point);
            args.push(arg);
        }
        args.push(raw_args);
        args.reverse();

        let mut final_args = HashMap::new();
        let arg_names = param_info.args();

        for (arg_name, arg) in arg_names.iter().zip(args.iter()) {
            final_args.insert(arg_name.to_string(), arg.clone());
        }
        if param_info.last_arg_is_catch_all() {
            let last_arg = args
                .iter()
                .skip(arg_names.len() - 1)
                .flatten()
                .cloned()
                .collect();
            final_args.insert(arg_names.last().unwrap().to_string(), last_arg);
        }

        let mut replacement_tokens = macro_definition.body().to_vec();
        let mut new_tokens = VecDeque::new();
        while !replacement_tokens.is_empty() {
            let token = replacement_tokens.remove(0);
            match token.value() {
                "##" => {
                    let name = replacement_tokens.remove(0);
                    let name = name.value();
                    if !final_args.contains_key(name) {
                        if param_info.last_arg_is_catch_all()
                            && param_info.args().last() == Some(&format!("{}...", name))
                        {
                            continue;
                        }
                        error!("`##` operator used on undefined argument `{}`", name);
                        return Err(ParseError::ERROR_MACRO_MALFORMED_CALL);
                    }
                    let arg = final_args.get(name).unwrap();
                    new_tokens.extend(arg.clone());
                }
                "#" => {
                    let name = replacement_tokens.remove(0);
                    let name = name.value();
                    if !final_args.contains_key(name) {
                        error!("`#` operator used on undefined argument `{}`", name);
                        return Err(ParseError::ERROR_MACRO_MALFORMED_CALL);
                    }
                    let arg = final_args.get(name).unwrap();
                    new_tokens.push_back(DmToken::from("\""));
                    new_tokens.extend(arg.clone());
                    new_tokens.push_back(DmToken::from("\""));
                }
                token if final_args.contains_key(token) => {
                    let arg = final_args.get(token).unwrap();
                    new_tokens.extend(arg.clone());
                }
                _ => {
                    new_tokens.push_back(token);
                }
            }
        }

        for token in new_tokens.into_iter().rev() {
            tokens.push_front(token);
        }

        Ok(None)
    }

    /// Prefer do_define_replacement wherever possible to not ignore defines as they get added.
    /// This should only be called in places where you know that no defines will be added.
    /// Such as inside of a macro or preprocessor directive parsing.
    pub fn replace_all_defines_possible(
        &self,
        tokens: &mut VecDeque<DmToken>,
        in_preprocessoer_directive: bool,
    ) -> Result<(), ParseError> {
        let mut return_tokens = vec![];

        while !tokens.is_empty() {
            let mut token = Some(tokens.pop_front().unwrap());

            match in_preprocessoer_directive {
                true if token.as_ref().unwrap().value() == "defined" => {
                    return_tokens.push(token.unwrap());
                    return_tokens.push(tokens.pop_front().unwrap()); // (
                    return_tokens.push(tokens.pop_front().unwrap()); // identifier
                    return_tokens.push(tokens.pop_front().unwrap()); // )
                    continue;
                }
                _ => {
                    token = self.do_define_replacement(token.unwrap(), tokens)?;
                }
            };

            if let Some(token) = token {
                return_tokens.push(token);
            }
        }

        tokens.extend(return_tokens);
        Ok(())
    }

    pub fn do_define_replacement(
        &self,
        token: DmToken,
        next_tokens: &mut VecDeque<DmToken>,
    ) -> Result<Option<DmToken>, ParseError> {
        let define = self.get_define(token.value());
        if define.is_none() {
            return Ok(Some(token));
        }

        let define = define.unwrap();
        if define.is_macro() {
            if !next_tokens.front().is_some_and(|tok| tok.value() == "(") {
                debug!("ignoring macro, no parenthesis");
                return Ok(Some(token));
            }
            debug!("macro `{}`", define.name());
            return Self::do_macro_replacement(define, next_tokens);
        }

        let tokens = define.body().to_vec();
        debug!("define replacement: `{}` `{:?}`", define.name(), &tokens);
        if tokens.is_empty() {
            return Ok(None);
        }

        next_tokens.reserve(tokens.len());
        for token in tokens.into_iter().rev() {
            next_tokens.push_front(token);
        }
        Ok(None)
    }
}
