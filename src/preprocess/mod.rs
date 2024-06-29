use std::collections::VecDeque;

use define::definition::DefineDefinition;
use environment::{CurrentFileEntry, EnvironmentData};
use line_processing::process_lines;
use log::error;

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
    pub fn new(working_directory: impl Into<String>, include_stddef: bool) -> Self {
        Self {
            environment: EnvironmentData::new(working_directory.into(), include_stddef),
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

    fn do_define_replace(&self, mut tokens: VecDeque<Token>) -> VecDeque<Token> {
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
            return tokens;
        }

        let mut splits: Vec<Vec<Token>> = vec![];
        for (replace, define) in define_match_indexes {
            let right_side = tokens.split_off(replace);
            splits.push(right_side.into_iter().skip(1).collect());
            splits.push(define.body().clone());
        }
        splits.push(tokens.into());
        splits.reverse();
        splits.concat().into_iter().collect()
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
        let mut final_raw: Vec<VecDeque<Token>> = vec![];
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
                final_raw.push(line_tokens.into());
            }
        }

        let mut final_preprocessed = vec![];
        for mut line in final_raw {
            // consume all whitespace at the start
            while line.front().is_some_and(|tok| tok.is_only_spacing()) {
                line.pop_front();
            }

            if !line.front().is_some_and(|tok| tok.value() == "#") {
                self.do_directive(line)?;
                continue;
            }

            if self.directive_skip_level() > 0 {
                continue;
            }

            line = self.do_define_replace(line);
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
