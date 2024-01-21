#![allow(dead_code)]
#![allow(unused)]

use dm_parser::DmParser;
use dotenv::dotenv;
use log::{info, logger, set_logger};

pub mod dm_parser;
pub mod dm_preprocessor;
pub mod util;

pub fn main() -> Result<(), String> {
    dotenv().ok();

    crate::util::log::init();

    info!("LiES -- Init");

    let mut parser = DmParser::new("D:/ss13/tgstation");
    parser.load("tgstation.dme")
}
