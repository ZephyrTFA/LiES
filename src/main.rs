use std::io::Error;

use dm_parser::DmParser;

pub mod dm_parser;
pub mod util;

pub fn main() -> Result<(), String> {
    let mut parser = DmParser::new("D:/ss13/tgstation");
    parser
        .load("tgstation.dme")
        .map_err(|err: Error| err.to_string())
}

pub fn is_verbose() -> bool {
    std::env::args().any(|arg| arg == "--verbose")
}
