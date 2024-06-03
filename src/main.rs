use dm_parser::lib::DmParser;
use dotenv::dotenv;
use log::{error, info, log, Level};
use std::env;

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

    log!(
        if result { Level::Info } else { Level::Error },
        "LiES -- {} in {:.3}s",
        if result { "Done" } else { "Failed" },
        stopwatch.elapsed().as_secs_f32()
    );
}

fn lies() -> bool {
    let game_dir = env::var("GAME_DIR").expect("GAME_DIR not set.");
    let dme_file = env::var("DME_FILE").expect("DME_FILE not set.");
    let mut parser = DmParser::new(game_dir);
    let result = parser.load_path(dme_file);

    if result.is_err() {
        let parse_error = result.as_ref().err().unwrap();
        error!("Error while parsing:");
        error!("\t{}", parse_error.to_string());
        if let Some(file_path) = parse_error.file_path() {
            let canonical = parser
                .environment_directory()
                .join(file_path)
                .canonicalize()
                .unwrap();
            error!(
                "\tat {}{}",
                canonical.display(),
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
    };

    result.is_ok()
}
