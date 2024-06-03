use std::{collections::VecDeque, rc::Rc};

#[cfg(test)]
use crate::util::dm_file::DmFile;
#[cfg(test)]
use std::error::Error;

use crate::{dm_parser::lib::DmParser, tokens::dm_token::DmToken, util::ParseError};

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

    let mut parser = DmParser::default();
    let file = DmFile {
        path: "test.dm".into(),
        lines: lines.into_iter().map(|s| s.into()).collect(),
    };

    parser.load_file(file)?;
    Ok(())
}

pub struct Scope {
    parent: Rc<Scope>,
}

impl DmParser {
    pub(super) fn parse_scopes(
        &mut self,
        mut _tokens: VecDeque<DmToken>,
    ) -> Result<VecDeque<Rc<Scope>>, ParseError> {
        unimplemented!()
    }
}
