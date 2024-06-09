pub mod preprocess;
pub mod tokenize;
pub mod util;

use dotenv::dotenv;
use log::{error, info, log, Level};
use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};
use tokenize::lib::tokenize_file;
use util::log::LOGGER;

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

    exit(if result { 0 } else { 1 });
}

fn lies() -> bool {
    let game_dir = env::var("GAME_DIR").expect("GAME_DIR not set.");
    let dme_file = env::var("DME_FILE").expect("DME_FILE not set.");

    // let mut parser = DmParser::new(game_dir);
    // let result = parser.load_path(dme_file);
    let result = tokenize_file(&Path::join(
        &PathBuf::from_str(&game_dir).unwrap(),
        PathBuf::from_str(&dme_file).unwrap(),
    ));

    if result.is_err() {
        let parse_error = result.as_ref().err().unwrap();
        error!("Error while parsing:");
        error!("\t{}", parse_error.to_string());
        if let Some(file_data) = parse_error.file_data() {
            error!(
                "\tat {}:({},{})",
                file_data.path(),
                file_data.line(),
                file_data.column(),
            );
        } else {
            error!("\\- at unknown location");
        }
    } else {
        info!("Success.");
    };
    info!(
        "Log file can be found at {}",
        LOGGER.get_log_file_full_path().display()
    );

    result.is_ok()
}
