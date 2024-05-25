use std::{collections::VecDeque, mem};

use crate::{dm_parser::lib::DmParser, tokens::dm_token::DmToken};

#[derive(Debug, Default)]
pub(super) struct Scope {
    type_path: Option<String>,
    lines: VecDeque<VecDeque<DmToken>>,
    indentation_level: usize,
}

#[test]
fn test_parse_scopes() {}

impl DmParser {
    pub(super) fn parse_scopes(&mut self, mut tokens: VecDeque<DmToken>) -> VecDeque<Scope> {
        let mut scopes = VecDeque::new();

        let mut brace_scope_level = 0;
        let mut current_line = VecDeque::new();
        let mut current_scope = Scope::default();

        let mut indentation_level: usize = 0;
        let mut type_cache = String::new();
        while !tokens.is_empty() {
            let token = tokens.pop_front().unwrap();
            if current_line.is_empty() && token.value().chars().all(char::is_whitespace) {
                indentation_level += token.value().len();
                continue;
            }

            if current_scope.indentation_level != indentation_level {
                scopes.push_back(mem::take(&mut current_scope));
                current_scope.indentation_level = indentation_level;
            }

            if current_scope.type_path.is_none() {
                match token.value() {
                    "var" | "proc" | "verb" => {
                        current_scope.type_path = Some(std::mem::take(&mut type_cache));
                    }
                    _ => {
                        type_cache.push_str(token.value());
                        continue;
                    }
                }
            }

            if brace_scope_level == 0 {
                match token.value() {
                    _ if token.is_in_string() => {
                        current_line.push_back(token);
                    }
                    ";" | "\n" => {
                        if !current_line.is_empty() {
                            current_scope.lines.push_back(mem::take(&mut current_line));
                        }
                        indentation_level = 0;
                    }
                    "{" => {
                        brace_scope_level += 1;
                    }
                    _ => {
                        current_line.push_back(token);
                    }
                }
            } else {
                todo!()
            }
        }

        scopes
    }
}
