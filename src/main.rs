use dm_parser::lib::DmParser;
use dotenv::dotenv;
use log::{error, info};
use std::env;
use util::ParseError;

pub mod dm_parser;
pub mod dm_preprocessor;
pub mod tokens;
pub mod util;

pub fn main() {
    dotenv().ok();

    crate::util::log::init();
    info!("LiES -- Init");

    let mut stopwatch = stopwatch::Stopwatch::start_new();
    let result = lies();
    stopwatch.stop();

    info!("LiES -- Done in {:.3}s", stopwatch.elapsed().as_secs_f32());
    if result.is_err() {
        let parse_error = result.err().unwrap();
        error!("Error: {}", parse_error.to_string());
        if parse_error.file_path().is_some() {
            error!(
                "\\- at {}{}",
                parse_error.file_path().unwrap(),
                parse_error
                    .line_number()
                    .map(|num| format!(":{num}"))
                    .unwrap_or_default()
            );
        } else {
            error!("\\- at unknown location");
        }
    } else {
        info!("Success.");
    }
}

fn lies() -> Result<(), ParseError> {
    let game_dir = env::var("GAME_DIR").unwrap();
    let mut parser = DmParser::new(game_dir);
    // parser.load(r"code\__HELPERS\visual_effects.dm")
    parser.load("tgstation.dme")
}
