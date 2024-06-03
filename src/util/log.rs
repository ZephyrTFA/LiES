use std::{
    env::{self, args},
    fs::{self, create_dir_all, DirEntry, File},
    io::{stdout, IsTerminal, Write},
    path::{Path, PathBuf},
};

use chrono::Local;
use log::{info, Level, LevelFilter, Log};
use once_cell::sync::Lazy;

struct InternalLogger {
    log_dir_path: PathBuf,
    log_file_path: PathBuf,
    log_file: File,
}

static LOGGER: Lazy<InternalLogger> = Lazy::new(|| {
    let (log_dir_path, log_file_path) = get_paths_dir_and_file();
    let log_file = get_file(&log_dir_path, &log_file_path);
    InternalLogger {
        log_dir_path,
        log_file_path,
        log_file,
    }
});

pub fn init() {
    let logger: &InternalLogger = &LOGGER;
    log::set_logger(logger).expect("failed to set logger");

    let mut highest_log_level = log::LevelFilter::Info;
    if let Ok(value) = env::var("LIES_LOG_LEVEL") {
        if let Ok(parsed) = value.parse::<i32>() {
            highest_log_level = match parsed {
                0 => LevelFilter::Error,
                1 => LevelFilter::Warn,
                2 => LevelFilter::Info,
                3 => LevelFilter::Debug,
                4 => LevelFilter::Trace,
                _ => {
                    eprintln!("Invalid value for LIES_LOG_LEVEL");
                    panic!();
                }
            }
        } else {
            match value.to_lowercase().as_str() {
                "error" => highest_log_level = LevelFilter::Error,
                "warn" => highest_log_level = LevelFilter::Warn,
                "info" => highest_log_level = LevelFilter::Info,
                "debug" => highest_log_level = LevelFilter::Debug,
                "trace" => highest_log_level = LevelFilter::Trace,
                _ => {
                    eprintln!("Invalid value for LIES_LOG_LEVEL");
                    panic!();
                }
            }
        }
        println!("Log level: {}", highest_log_level);
    } else {
        for arg in args() {
            match arg.as_str() {
                "--verbose" | "--debug" if highest_log_level < LevelFilter::Debug => {
                    highest_log_level = log::LevelFilter::Debug
                }
                "--trace" if highest_log_level < LevelFilter::Trace => {
                    highest_log_level = log::LevelFilter::Trace
                }
                _ => {}
            }
        }
    }
    log::set_max_level(highest_log_level);
    info!("Log Directory: {}", logger.log_dir_path.display());
    info!("Log File: {}", logger.log_file_path.display());
}

fn get_paths_dir_and_file() -> (PathBuf, PathBuf) {
    let log_directory = env::var("LIES_LOG_DIR").unwrap_or("./logs".to_string());
    create_dir_all(&log_directory).expect("failed to create log dir");
    let log_directory = Path::canonicalize(&PathBuf::from(log_directory)).unwrap();

    let current_time = Local::now();
    let log_file_path: PathBuf = current_time.format("%Y%m%d_%H%M%S.log").to_string().into();

    (log_directory, log_file_path)
}

fn get_file(dir: &PathBuf, file: &PathBuf) -> File {
    let mut old_logs: Vec<DirEntry> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "log")
        })
        .collect();
    old_logs.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .created()
            .unwrap()
            .cmp(&a.metadata().unwrap().created().unwrap())
    });

    if old_logs.len() > 5 {
        for old_log in &old_logs[5..] {
            println!(
                "Removing old log `{}`",
                old_log.file_name().to_str().unwrap()
            );
            fs::remove_file(old_log.path()).expect("failed to delete old log file");
        }
    }

    File::create_new(dir.join(file)).expect("failed to create log file")
}

impl InternalLogger {
    fn color(&self, level: Level, message: &str) -> String {
        if !self.color_enabled() {
            return message.to_string();
        }

        let color = match level {
            Level::Error => "\x1b[31m",
            Level::Warn => "\x1b[33m",
            Level::Info => "\x1b[32m",
            Level::Debug => "\x1b[34m",
            Level::Trace => "\x1b[35m",
        };

        format!("{}{}{}", color, message, "\x1b[0m")
    }

    fn color_enabled(&self) -> bool {
        stdout().is_terminal()
    }
}

impl Log for InternalLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let record_body = record.args().to_string();
        let timestamp = chrono::Local::now().format("%H:%M:%S");
        let colored = self.color(record.level(), &record_body);
        let headers = format!("[{}][{}]", timestamp, record.level(),);

        // Trace output will never go to stdout
        if record.level() < Level::Trace {
            let mut stdout = stdout();
            stdout.write_all(headers.as_bytes()).unwrap();
            stdout.write_all(b" ").unwrap();
            stdout.write_all(colored.as_bytes()).unwrap();
            stdout.write_all(b"\n").unwrap();
            stdout.flush().expect("failed to write to stdout");
        }
        let mut log_file = self.log_file.try_clone().unwrap();
        log_file.write_all(headers.as_bytes()).unwrap();
        log_file.write_all(b" ").unwrap();
        log_file.write_all(record_body.as_bytes()).unwrap();
        log_file.write_all(b"\n").unwrap();
        log_file.flush().expect("failed to write to file");
    }

    fn flush(&self) {
        self.log_file
            .try_clone()
            .unwrap()
            .flush()
            .expect("failed to flush file buffer");
    }
}
