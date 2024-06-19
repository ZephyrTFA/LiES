use std::iter::Peekable;

use environment::{CurrentFileEntry, EnvironmentData};
use line_processing::process_lines;

use crate::{
    tokenize::{lib::tokenize_file, token::Token},
    util::parse_error::ParseError,
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

    fn do_define_replace<'a>(
        &self,
        tokens: Peekable<impl Iterator<Item = &'a Token>>,
    ) -> impl Iterator<Item = Token> {
        tokens.cloned()
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

            let mut line = self.do_define_replace(line).peekable();
            if line.peek().is_some_and(|tok| tok.value() == "#") {
                self.do_directive(line)?;
                // directives always consume the entire line
                continue;
            }

            if self.directive_skip_level() > 0 {
                continue;
            }

            final_preprocessed.push(line);
        }

        self.environment_mut().pop_current_file();
        Ok(())
    }
}
