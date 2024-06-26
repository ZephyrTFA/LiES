use define::definition::DefineDefinition;
use environment::{CurrentFileEntry, EnvironmentData};
use line_processing::process_lines;
use log::{debug, error};

use crate::{
    tokenize::{lib::tokenize_file, token::Token},
    util::parse_error::{ParseError, ParseErrorCode},
};

pub mod define;
pub mod directive;
pub mod environment;
pub mod line_processing;
pub mod path;
pub mod string_tokens;

pub struct PreprocessState {
    environment: EnvironmentData,
    directive_skip_level: usize,
}

impl PreprocessState {
    pub fn new(working_directory: impl Into<String>) -> Self {
        Self {
            environment: EnvironmentData::new(working_directory.into()),
            directive_skip_level: 0,
        }
    }

    pub fn directive_skip_level(&self) -> usize {
        self.directive_skip_level
    }

    pub fn increment_directive_skip_level(&mut self) {
        self.directive_skip_level += 1;
    }

    pub fn decrement_directive_skip_level(&mut self) {
        self.directive_skip_level -= 1;
    }

    pub fn environment(&self) -> &EnvironmentData {
        &self.environment
    }

    pub fn environment_mut(&mut self) -> &mut EnvironmentData {
        &mut self.environment
    }

    fn do_define_replace(&self, mut tokens: Vec<&Token>, _is_directive: bool) -> Vec<Token> {
        let defines = self.environment().defines();
        let define_match_indexes: Vec<(usize, &DefineDefinition)> = tokens
            .iter()
            .enumerate()
            .map(|(index, token)| (index, defines.get_define(token.value())))
            .filter(|(_, define_option)| define_option.is_some())
            .map(|(index, define_option)| (index, define_option.unwrap()))
            .rev()
            .collect();

        if define_match_indexes.is_empty() {
            return tokens.into_iter().cloned().collect();
        }

        let mut splits = vec![];
        for (replace, define) in define_match_indexes {
            splits.push(tokens.split_off(replace));
            splits.push(define.body().iter().collect());
        }
        splits.push(tokens);

        splits.concat().into_iter().cloned().collect()
    }

    pub fn preprocess(&mut self, file: &str) -> Result<(), ParseError> {
        // sanitize file path here
        let file = file.replace('\\', "/");

        let actual_path = self.process_file_path(&file);
        self.environment_mut()
            .push_current_file(CurrentFileEntry::new(&file, actual_path.to_str().unwrap()));
        let tokens = tokenize_file(&actual_path)?;
        let lines = process_lines(&tokens);

        // drop normal comments
        let mut final_raw: Vec<Vec<Token>> = vec![];
        for line in lines {
            let mut line_tokens = vec![];
            for token in line {
                let token_value = token.value();
                if token_value == "//" {
                    break;
                }
                line_tokens.push(token.clone());
            }
            if !line_tokens.is_empty() {
                final_raw.push(line_tokens);
            }
        }

        let mut final_preprocessed = vec![];
        for mut line in final_raw.iter().map(|tokens| tokens.iter().peekable()) {
            // consume all whitespace at the start
            while line.peek().is_some_and(|tok| tok.is_only_spacing()) {
                line.next().unwrap();
            }

            let is_directive = line.peek().is_some_and(|tok| tok.value() == "#");
            let line = self.do_define_replace(line.collect(), is_directive);
            if is_directive {
                debug!("{line:?}");
                self.do_directive(line.into_iter().peekable())?;
                // directives always consume the entire line
                continue;
            }

            if self.directive_skip_level() > 0 {
                continue;
            }

            final_preprocessed.push(line);
        }

        if self.directive_skip_level != 0 {
            error!("EOF with remaining directive skip level. Missing #endif?");
            return Err(ParseError::new(ParseErrorCode::UnexpectedEOL));
        }
        self.environment_mut().pop_current_file();
        Ok(())
    }
}
