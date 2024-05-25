use std::collections::VecDeque;

#[cfg(test)]
use crate::util::dm_file::DmFile;
#[cfg(test)]
use std::error::Error;

use crate::{dm_parser::lib::DmParser, tokens::dm_token::DmToken};

#[derive(Debug, Default)]
pub(super) struct Scope {
    type_path: Option<String>,
    lines: VecDeque<VecDeque<DmToken>>,
    indentation_level: usize,
}

#[test]
fn test_parse_scopes() -> Result<(), Box<dyn Error>> {
    let lines = vec![
        "/proc/global_proc(param, var/byond_style_param, atom/typed_param, var/atom/byond_style_typed_param)",
        "    return",
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
    pub(super) fn parse_scopes(&mut self, mut tokens: VecDeque<DmToken>) -> VecDeque<Scope> {
        let mut _scopes = VecDeque::new();

        while !tokens.is_empty() {
            _ = tokens.pop_front().unwrap();
        }

        _scopes
    }
}
