#![allow(dead_code)]
#![allow(unused)]

use dm_parser::DmParser;
use log::{logger, set_logger};

pub mod dm_parser;
pub mod dm_preprocessor;
pub mod util;

pub fn main() -> Result<(), String> {
    crate::util::log::init();

    let mut parser = DmParser::new("D:/ss13/tgstation");
    parser.load("tgstation.dme")

    // Ok(())
}
