use std::{
    fs,
    path::{Path, PathBuf},
};

use dm_parser::DmParser;
use dotenv::dotenv;
use log::info;

pub mod dm_parser;
pub mod dm_preprocessor;
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
        "\t. = list({\"",
        "\t\t<a href='?_src_=holder;[HrefToken()];stickyban=remove&ckey=[ckey]'>\\[-\\]</a>",
        "\t\t[timeout]",
        "\t\t<b>[ckey]</b>",
        "\t\t<br />\"",
        "\t\t[ban[\"message\"]] <b><a href='?_src_=holder;[HrefToken()];stickyban=edit&ckey=[ckey]'>\\[Edit\\]</a></b><br />",
        "\t\"})",
    ];

    let test_case_file = Path::new(&WORK_DIR).join("test_case.dm");
    println!("TCF: {test_case_file:?}");
    fs::write(&test_case_file, test_case.join("\n")).expect("failed to write test case file");
    test_case_file
}
