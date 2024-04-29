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

    let mut stopwatch = stopwatch::Stopwatch::start_new();

    let mut parser = DmParser::new(WORK_DIR);
    parser.load("tgstation.dme")?;

    stopwatch.stop();
    info!("LiES -- Done in {:.3}s", stopwatch.elapsed().as_secs_f32());
    Ok(())
}
