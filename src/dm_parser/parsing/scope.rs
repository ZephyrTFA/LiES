use std::{collections::VecDeque, rc::Rc};

#[cfg(test)]
use crate::util::dm_file::DmFile;
#[cfg(test)]
use std::error::Error;

use crate::{dm_parser::lib::DmParser, tokens::dm_token::DmToken, util::ParseError};

#[derive(Debug, Default)]
pub(super) struct Scope {
    type_path: Option<String>,
    tokens: VecDeque<DmToken>,
    indentation_level: usize,
    parent: Option<Rc<Scope>>,
}

#[test]
fn test_parse_scopes() -> Result<(), Box<dyn Error>> {
    let lines = vec![
        "/proc/global_proc(param, var/byond_style_param, atom/typed_param, var/atom/byond_style_typed_param)",
        "  return",
        "/obj",
        "  var",
        "    obj_var = 2",
        "  proc",
        "    obj_proc()",
        "      return",
        "  subtype",
        "    var",
        "      subtype_var = 5",
        "    proc",
        "      subtype_proc()",
        "        return",
        "    obj_proc()",
        "      return",
        "/turf/icon_state = \"state_here[2]\" + 45",
    ];

    let mut parser = DmParser::default();
    let file = DmFile {
        path: "test.dm".into(),
        lines: lines.into_iter().map(|s| s.into()).collect(),
    };

    parser.load_file(file)?;
    Ok(())
}

const IDENT_VAR: &str = "var";
const IDENT_PROC: &str = "proc";
const IDENT_VERB: &str = "verb";

const IDENTS: [&str; 3] = [IDENT_VAR, IDENT_PROC, IDENT_VERB];

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
            let token_value = token.value();
            if identation_level.is_none() {
                if token_value.chars().all(char::is_whitespace) {
                    total_whitespace += token_value.len();
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
            if token_value.chars().all(|c| [' ', '\t'].contains(&c)) {
                continue;
            }

            if token_value == "\n" {
                identation_level = None;
                continue;
            }

            if token_indent_level != current_scope.indentation_level {
                let old_scope = std::mem::take(&mut current_scope);
                scope_vec.push_back(old_scope.into());
                let old_scope = scope_vec.back().unwrap();

                let old_identation_level = old_scope.indentation_level;
                current_scope.indentation_level = token_indent_level;
                let indent_difference = token_indent_level as isize - old_identation_level as isize;
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
                    current_scope.parent = Some(parent_scope);
                }
            }
        }

        Ok(scope_vec)
    }
}
