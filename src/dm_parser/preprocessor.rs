use std::{
    collections::HashMap,
    io::{Error, Result},
    path::PathBuf,
};

use crate::is_verbose;

use super::{define_definition::DefineDefinition, token_store::TokenStore};

use super::DmParser;

pub struct PreprocessState<'a> {
    pub defines: HashMap<String, DefineDefinition>,
    pub skip_until_token_matches: Option<&'a str>,
    pub skip_levels: usize,
}

impl PreprocessState<'_> {
    pub fn new() -> Self {
        Self {
            defines: Self::populate_initial_defines(),
            skip_until_token_matches: None,
            skip_levels: 0,
        }
    }

    pub fn base_file_dir(&self) -> PathBuf {
        let file_dir_define = self.defines.get("BASE_FILE_DIR");
        if file_dir_define.is_none() {
            return PathBuf::from(".");
        }
        return PathBuf::from(file_dir_define.unwrap().body.as_ref().unwrap());
    }

    fn populate_initial_defines() -> HashMap<String, DefineDefinition> {
        let mut defines = HashMap::new();
        DefineDefinition::new_basic_replace("TRUE", "1").insert_into_map(&mut defines);
        DefineDefinition::new_basic_replace("FALSE", "0").insert_into_map(&mut defines);
        DefineDefinition::new_basic_replace("DM_VERSION", "515").insert_into_map(&mut defines);
        DefineDefinition::new_basic_replace("DM_BUILD", "1624").insert_into_map(&mut defines);
        return defines;
    }
}

impl DmParser<'_> {
    fn split_line_into_tokens(line: &str) -> Vec<String> {
        if line.is_empty() {
            return vec![];
        }

        const SPLIT_AT: &[char; 9] = &[' ', '\t', '(', ')', '"', '!', ';', '.', '/'];
        let raw_split: Vec<&str> = line.split_inclusive(SPLIT_AT).collect();

        // go through tokens and remove trailing split markers into their own tokens
        let mut final_tokens: Vec<&str> = vec![];
        for i in 0..raw_split.len() {
            let token = raw_split[i];
            if token.len() == 1 {
                final_tokens.push(token);
                continue;
            }

            let last = token.chars().last().unwrap();
            if !SPLIT_AT.contains(&last) {
                final_tokens.push(token);
                continue;
            }

            let (token, final_char) = token.split_at(token.len() - 1);
            final_tokens.push(token);
            final_tokens.push(final_char);
        }

        let mut processed_tokens: Vec<String> = vec![];
        let mut in_string = false;
        // go through tokens and combine tokens which should be combined. (strings, etc)
        for idx in 0..final_tokens.len() {
            let token = final_tokens[idx];

            match token.chars().next().unwrap() {
                '/' => {
                    if idx != 0 {
                        let mut last = processed_tokens.pop().unwrap();
                        last.push_str(token);
                        processed_tokens.push(last);
                    } else {
                        processed_tokens.push(token.to_string());
                    }
                }
                '"' => {
                    in_string = !in_string;
                    processed_tokens.push(token.to_string());
                }
                _ => {
                    if in_string && processed_tokens.last().unwrap().chars().next().unwrap() != '"'
                    {
                        let mut last = processed_tokens.pop().unwrap();
                        last.push_str(token);
                        processed_tokens.push(last);
                        continue;
                    }
                    processed_tokens.push(token.to_string());
                }
            }
        }

        processed_tokens
    }

    pub(super) fn preprocess(&mut self, path: &PathBuf, lines: Vec<String>) -> Result<Vec<String>> {
        if path.extension().unwrap() == "dmm" {
            return Ok(lines);
        }

        let mut processed = vec![];

        'main_loop: for line in lines {
            let mut processed_line: Vec<TokenStore> = vec![];

            for token in Self::split_line_into_tokens(&line) {
                let token = TokenStore {
                    raw: token.to_string(),
                    processed: token.trim().to_string(),
                };

                if token.processed.is_empty() {
                    continue;
                }

                let token = self.process_token_replacement(token);
                if self.preprocess_state.skip_levels > 0 && !self.process_skip_depth(&token) {
                    continue 'main_loop;
                }

                if self.preprocess_state.skip_until_token_matches.is_some() {
                    if token.processed == self.preprocess_state.skip_until_token_matches.unwrap() {
                        self.preprocess_state.skip_until_token_matches = None;
                    }
                    continue 'main_loop;
                }

                if token.processed.starts_with("//") {
                    continue 'main_loop;
                } else if token.processed.starts_with("/*") {
                    self.preprocess_state.skip_until_token_matches = Some(r"\*/$");
                    continue;
                }

                processed_line.push(token);
            }

            if processed_line.is_empty() {
                continue;
            }

            if processed_line[0].processed.starts_with("#") {
                let result =
                    self.parse_preprocessor_directive(&processed_line[0], &processed_line[1..]);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
                continue;
            }

            let processed_line = processed_line
                .iter()
                .map(|token| token.raw.as_str())
                .collect::<Vec<&str>>()
                .join("");
            println!("PreProcessed Line: `{}`", processed_line);
            processed.push(processed_line);
        }

        if self.preprocess_state.skip_until_token_matches.is_some() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "unterminated token pair: {}",
                    self.preprocess_state.skip_until_token_matches.unwrap()
                ),
            ));
        }

        if self.preprocess_state.skip_levels > 0 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "unterminated preprocess skip level",
            ));
        }

        Ok(processed)
    }

    fn process_skip_depth(&mut self, token: &TokenStore) -> bool {
        const LOGICAL_DIRECTIVE_START: &[&str] = &["#if", "#ifdef", "#ifndef"];
        const LOGICAL_DIRECTIVE_END: &[&str] = &["#endif"];

        if is_verbose() {
            println!("checking for skip token change: `{}`", token.raw);
        }

        if LOGICAL_DIRECTIVE_START.contains(&token.processed.as_str()) {
            self.preprocess_state.skip_levels += 1;
            if is_verbose() {
                println!("increasing skip level");
            }
        } else if LOGICAL_DIRECTIVE_END.contains(&token.processed.as_str()) {
            self.preprocess_state.skip_levels -= 1;
            if is_verbose() {
                println!("decreasing skip level");
            }
        }

        if is_verbose() {
            println!("skip level: {}", self.preprocess_state.skip_levels);
        }
        self.preprocess_state.skip_levels == 0
    }

    fn process_token_replacement(&self, token: TokenStore) -> TokenStore {
        if !self.preprocess_state.defines.contains_key(&token.processed) {
            return token;
        }

        let define = self.preprocess_state.defines.get(&token.processed).unwrap();
        if define.body.is_none() {
            return token;
        }

        if define.is_macro {
            todo!()
        }

        let replaced = token
            .raw
            .replacen(&token.processed, define.body.as_ref().unwrap(), 1);
        TokenStore {
            processed: replaced.trim().to_string(),
            raw: replaced,
        }
    }

    fn parse_preprocessor_directive(
        &mut self,
        directive: &TokenStore,
        tokens: &[TokenStore],
    ) -> Result<()> {
        let directive = &directive.processed;
        if !directive.starts_with("#") {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "not a preprocessor directive",
            ));
        }

        match &directive[1..] {
            "include" => self.parse_preprocessor_include(tokens),
            "define" => self.parse_preprocessor_define(tokens),
            "ifndef" => self.parse_prepocessor_ifndef(tokens),
            "ifdef" => self.parse_prepocessor_ifdef(tokens),
            "if" => self.parse_preprocessor_if(tokens),
            "warn" => self.preprocessor_warn(tokens),
            "error" => self.preprocessor_error(tokens),
            "endif" => Ok(()),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid preprocessor directive: `{}`", directive),
            )),
        }
    }

    fn parse_preprocessor_include(&mut self, tokens: &[TokenStore]) -> Result<()> {
        if tokens.len() < 3 {
            eprintln!("include tokens: {:?}", tokens);
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid include format; not enough tokens",
            ));
        }

        if tokens[0].processed != "\"" || tokens[2].processed != "\"" {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid include format; expected quotes",
            ));
        }

        self.load(&tokens[1].processed)
    }

    fn parse_preprocessor_define(&mut self, tokens: &[TokenStore]) -> Result<()> {
        let name: &String = &tokens[0].processed;
        let initial_non_whitespace = tokens
            .iter()
            .skip(1)
            .position(|token| !token.processed.is_empty());

        if initial_non_whitespace.is_none() {
            DefineDefinition::new_flag(name).insert_into_map(&mut self.preprocess_state.defines);
            return Ok(());
        }
        let initial_non_whitespace = tokens[initial_non_whitespace.unwrap()].processed.as_str();

        match initial_non_whitespace {
            "(" => {
                panic!()
            }
            _ => {
                let mut body = String::new();
                for token in &tokens[1..] {
                    body.push_str(&token.raw);
                }
                DefineDefinition::new_basic_replace(name, body)
                    .insert_into_map(&mut self.preprocess_state.defines);
                return Ok(());
            }
        }
    }

    fn parse_prepocessor_ifndef(&mut self, tokens: &[TokenStore]) -> Result<()> {
        if tokens.len() > 1 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid ifndef format; too many tokens",
            ));
        }

        if self
            .preprocess_state
            .defines
            .contains_key(&tokens[0].processed)
        {
            self.preprocess_state.skip_levels += 1;
            if is_verbose() {
                println!(
                    "ifndef failed: `{}` skip level incremented",
                    tokens[0].processed
                )
            }
        }

        Ok(())
    }

    fn parse_prepocessor_ifdef(&mut self, tokens: &[TokenStore]) -> Result<()> {
        if tokens.len() > 1 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid ifdef format; too many tokens",
            ));
        }

        if !self
            .preprocess_state
            .defines
            .contains_key(&tokens[0].processed)
        {
            self.preprocess_state.skip_levels += 1;
            if is_verbose() {
                println!(
                    "ifdef failed: `{}` skip level incremented",
                    tokens[0].processed
                )
            }
        }

        Ok(())
    }

    fn preprocessor_warn(&self, tokens: &[TokenStore]) -> Result<()> {
        let mut message = String::new();
        for token in tokens {
            message.push_str(&token.raw);
        }
        eprintln!("Preprocessor Warning: {}", message);
        Ok(())
    }

    fn preprocessor_error(&self, tokens: &[TokenStore]) -> Result<()> {
        let mut message = String::new();
        for token in tokens {
            message.push_str(&token.raw);
        }
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Preprocessor Error: {}", message),
        ))
    }
}
