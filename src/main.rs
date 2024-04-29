use std::{
    fs,
    path::{Path, PathBuf},
};

use dm_parser::lib::DmParser;
use dotenv::dotenv;
use log::info;

pub mod dm_parser;
pub mod dm_preprocessor;
pub mod tokens;
pub mod util;

const WORK_DIR: &str = "D:/ss13/tgstation";

pub fn main() -> Result<(), String> {
    dotenv().ok();

    crate::util::log::init();

    info!("LiES -- Init");

    let mut parser = DmParser::new(WORK_DIR);
    // parser.load("tgstation.dme")
    parser.load(get_test_case())
}

fn get_test_case() -> PathBuf {
    let test_case: Vec<&str> = vec![
        "var/regex/admin_ranks_regex = new(@\"^Name\\s*=\\s*(.+?)\\s*\\n+Include\\s*=\\s*([\\l @]*?)\\s*\\n+Exclude\\s*=\\s*([\\l @]*?)\\s*\\n+Edit\\s*=\\s*([\\l @]*?)\\s*\\n*$\", \"gm\")"
    ];

    let test_case_file = Path::new(&WORK_DIR).join("test_case.dm");
    println!("TCF: {test_case_file:?}");
    fs::write(&test_case_file, test_case.join("\n")).expect("failed to write test case file");
    test_case_file
}
