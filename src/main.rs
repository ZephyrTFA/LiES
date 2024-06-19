pub mod preprocess;
pub mod tokenize;
pub mod util;

use dotenv::dotenv;
use log::{debug, error, info, log, Level};
use std::{env, path::Path, process::exit};
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

    let mut parser = preprocess::PreprocessState::new(&game_dir);
    let entry_path = Path::new(&game_dir).join(&dme_file);
    let entry_path = entry_path
        .to_str()
        .expect("failed to parse entry file as path");
    let result = parser.preprocess(entry_path);

    if result.is_ok() {
        debug!("{:?}", result.as_ref().unwrap());
        info!("Success.");
    } else {
        let parse_error = result.as_ref().err().unwrap();
        error!("Error while parsing:");
        error!("\t{}", parse_error.to_string());
        if let Some(file_data) = parse_error.file_data() {
            error!(
                "\tat {}:{}:{}",
                file_data.full_path(),
                file_data.line() + 1, // line and columns references start at 1 not 0
                file_data.column() + 1,
            );
        } else {
            error!("\\- at unknown location");
        }
    }
    info!("Environment: {game_dir}");
    info!("Entry: {dme_file} ({game_dir}/{dme_file})");
    info!(
        "Log file can be found at {}",
        LOGGER.get_log_file_full_path().display()
    );

    result.is_ok()
}
