use std::{iter::Peekable, rc::Rc};

use log::error;

#[cfg(test)]
use crate::util::dm_file::DmFile;
#[cfg(test)]
use std::error::Error;

use crate::{tokens::dm_token::DmToken, util::ParseError};

use super::type_path::DmTypePath;

#[test]
fn test_parse_scopes() -> Result<(), Box<dyn Error>> {
    let lines = vec![
        "/proc/global_proc(param, var/byond_style_param, atom/typed_param = new, var/atom/byond_style_typed_param = new /atom/gay)",
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
        "    proc/do_thing()",
        "      return",
        "    proc",
        "      subtype_proc()",
        "        return",
        "    obj_proc()",
        "      return",
        "/turf/icon_state = \"state_here[2]\" + 45",
    ];

    let mut parser = crate::dm_parser::lib::DmParser::default();
    let file = DmFile {
        path: "test.dm".into(),
        lines: lines.into_iter().map(|s| s.into()).collect(),
    };

    parser.load_file(file)?;
    Ok(())
}

#[derive(Default)]
pub struct Scope {
    parent: Option<Rc<Scope>>,
    scope_type_path: Option<DmTypePath>,
    effective_type_path: Option<DmTypePath>,
    indentation_level: Option<usize>,
}

impl Scope {
    pub fn set_parent(&mut self, parent: Rc<Scope>) -> Result<(), ParseError> {
        if self.parent.is_some() {
            error!("attempt to set parent twice in scope");
            return Err(ParseError::INTERNAL_ERROR);
        }

        self.parent = Some(parent);
        Ok(())
    }

    pub fn parent(&self) -> Option<&Rc<Scope>> {
        self.parent.as_ref()
    }

    pub fn type_path(&self) -> Option<&DmTypePath> {
        self.scope_type_path.as_ref()
    }

    pub fn effective_type_path(&self) -> Option<&DmTypePath> {
        self.effective_type_path.as_ref()
    }

    pub fn consume_type_path(
        &mut self,
        tokens: &mut Peekable<impl Iterator<Item = DmToken>>,
    ) -> Result<(), ParseError> {
        if self.scope_type_path.is_some() {
            error!("attempting to set scope type path twice");
            return Err(ParseError::INTERNAL_ERROR);
        }
        self.set_scope_type_path(DmTypePath::consume_from_tokens(tokens)?);
        Ok(())
    }

    fn set_scope_type_path(&mut self, scope_type_path: DmTypePath) {
        if self.scope_type_path.is_some() {
            panic!("attempt to set scope type path twice");
        }

        if let Some(parent) = &self.parent {
            self.effective_type_path = Some(
                parent
                    .effective_type_path
                    .as_ref()
                    .unwrap()
                    .join(&scope_type_path),
            );
        } else {
            self.effective_type_path = Some(scope_type_path.clone());
        }
        self.scope_type_path = Some(scope_type_path);
    }

    pub fn set_indentation_level(&mut self, level: usize) {
        if self.indentation_level.is_some() {
            panic!("attempt to set indentation level twice");
        }
        self.indentation_level = Some(level)
    }

    pub fn indentation_level(&self) -> Option<usize> {
        self.indentation_level
    }
}
