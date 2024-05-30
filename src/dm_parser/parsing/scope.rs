use std::{collections::VecDeque, rc::Rc};

#[cfg(test)]
use crate::util::dm_file::DmFile;
#[cfg(test)]
use std::error::Error;

use crate::{
    dm_parser::lib::DmParser,
    tokens::dm_token::DmToken,
    util::{is_valid_identifier, ParseError},
};

use super::type_path::DmTypePath;

#[derive(Debug, Default)]
pub(super) struct Scope {
    type_path: Option<DmTypePath>,
    tokens: VecDeque<DmToken>,
    indentation_level: usize,
    parent: Option<Rc<Scope>>,
}

impl Scope {
    pub fn set_type_path(&mut self, type_path: DmTypePath) {
        self.type_path = Some(type_path);
    }

    pub fn type_path(&self) -> Option<&DmTypePath> {
        self.type_path.as_ref()
    }

    pub fn parent(&self) -> Option<&Rc<Scope>> {
        self.parent.as_ref()
    }
}

#[test]
fn test_parse_scopes() -> Result<(), Box<dyn Error>> {
    let lines = vec![
        "/proc/global_proc(param, var/byond_style_param, atom/typed_param = new, var/atom/byond_style_typed_param = new /atom/gay)",
        // "  return",
        // "/obj",
        // "  var",
        // "    obj_var = 2",
        // "  proc",
        // "    obj_proc()",
        // "      return",
        // "  subtype",
        // "    var",
        // "      subtype_var = 5",
        // "    proc",
        // "      subtype_proc()",
        // "        return",
        // "    obj_proc()",
        // "      return",
        // "/turf/icon_state = \"state_here[2]\" + 45",
    ];

    let mut parser = DmParser::default();
    let file = DmFile {
        path: "test.dm".into(),
        lines: lines.into_iter().map(|s| s.into()).collect(),
    };

    parser.load_file(file)?;
    Ok(())
}

impl DmParser {
    pub(super) fn parse_scopes(
        &mut self,
        mut tokens: VecDeque<DmToken>,
    ) -> Result<VecDeque<Rc<Scope>>, ParseError> {
        let mut scope_vec: VecDeque<Rc<Scope>> = VecDeque::new();

        let mut current_scope = Scope::default();

        // current indentation level, or None if not yet calculated
        let mut identation_level = None;
        // the total number of whitespace characters encountered so far
        let mut total_whitespace = 0;
        // the number of whitespace chars that make up a single indentation level
        let mut indentation_divisor = 0;

        while let Some(token) = tokens.pop_front() {
            println!("token: {}", token.value().escape_debug());
            if identation_level.is_none() {
                if token.is_only_whitespace(false) {
                    total_whitespace += token.value().len();
                    continue;
                } else {
                    // non-whitespace token, set the identation level and reset the whitespace counter
                    // but only if we're not a global scope
                    if total_whitespace > 0 {
                        if indentation_divisor == 0 {
                            indentation_divisor = total_whitespace;
                        }
                        identation_level = Some(total_whitespace / indentation_divisor);
                        total_whitespace = 0;
                    } else {
                        identation_level = Some(0);
                    }
                }
            }

            let token_indent_level = identation_level.unwrap();
            if token.is_in_string() {
                current_scope.tokens.push_back(token);
                continue;
            }

            // ignore empty tokens entirely
            if token.is_only_whitespace(false) {
                continue;
            }

            if token.value() == "\n" {
                identation_level = None;
                continue;
            }

            if token_indent_level != current_scope.indentation_level {
                println!(
                    "indentation level changed. old: {}, new: {}",
                    current_scope.indentation_level, token_indent_level
                );
                let old_scope = std::mem::take(&mut current_scope);
                scope_vec.push_back(old_scope.into());
                let old_scope = scope_vec.back().unwrap();

                let old_identation_level = old_scope.indentation_level;
                current_scope.indentation_level = token_indent_level;
                if current_scope.indentation_level == 0 {
                    current_scope.parent = None; // global
                } else {
                    let indent_difference =
                        token_indent_level as isize - old_identation_level as isize;
                    if indent_difference >= 2 {
                        return Err(ParseError::MISMATCHED_INDENTATION_COUNT);
                    } else if indent_difference == 1 {
                        current_scope.parent = Some(Rc::clone(old_scope));
                    } else {
                        assert!(indent_difference < 0);
                        let mut parent_scope = Rc::clone(old_scope);
                        for _ in 0..-indent_difference {
                            parent_scope = Rc::clone(parent_scope.parent.as_ref().unwrap());
                        }
                        println!(
                            "parent typepath: {}",
                            parent_scope.type_path.as_ref().unwrap()
                        );
                        current_scope.parent = Some(parent_scope);
                    }
                }
            }

            if current_scope.type_path().is_none() {
                self.consume_scope_typepath(token, &mut tokens, &mut current_scope)?;
                if let Some(second_last_part) = current_scope
                    .type_path()
                    .unwrap()
                    .parts()
                    .iter()
                    .rev()
                    .nth(1)
                {
                    match *second_last_part {
                        "proc" => {
                            println!("inline proc decl");
                            self.consume_proc(&mut tokens, &mut current_scope)?;
                        }
                        "var" => {
                            println!("inline var decl");
                        }
                        _ => {}
                    }
                }
                continue;
            }
        }

        Ok(scope_vec)
    }

    fn consume_until_non_whitespace(tokens: &mut VecDeque<DmToken>, consume_newlines: bool) {
        while tokens
            .front()
            .is_some_and(|t| t.is_only_whitespace(consume_newlines))
        {
            tokens.pop_front();
        }
    }

    fn consume_ident_name(tokens: &mut VecDeque<DmToken>) -> Result<String, ParseError> {
        let ident_name = tokens
            .pop_front()
            .ok_or(ParseError::UNEXPECTED_EOL)?
            .value()
            .to_string();
        if !is_valid_identifier(&ident_name) {
            return Err(ParseError::INVALID_IDENTIFIER);
        }

        Ok(ident_name)
    }
}
