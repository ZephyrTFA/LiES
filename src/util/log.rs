use std::{
    env::{self, args},
    fs,
    io::{self, IsTerminal, Stdout, Write},
    path::{Path, PathBuf},
    process::exit,
};

use log::{Level, LevelFilter, Log};
use once_cell::sync::Lazy;
use regex::Replacer;

#[derive(Default)]
struct InternalLogger {
    log_stdout: Option<fs::File>,
}

pub fn init() {
    const LOGGER: Lazy<InternalLogger> = Lazy::new(|| InternalLogger::default());
    log::set_logger(Lazy::get(&LOGGER).expect("FAILED TO INIT LOGGER")).unwrap();

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
        println!(
            "Overriding log level due to environment variable LIES_LOG_LEVEL to {}",
            highest_log_level
        );
    } else {
        for arg in args() {
            match arg.as_str() {
                ("--verbose" | "--debug") if highest_log_level < LevelFilter::Debug => {
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
    LOGGER.setup_log_file();
}

impl InternalLogger {
    fn get_log_file_target() -> PathBuf {
        let mut args = args();
        while let Some(arg) = args.next() {
            if arg == "--log-file" {
                if let Some(path) = args.next() {
                    return Path::new(&path).to_path_buf();
                } else {
                    eprintln!("Missing argument for --log-file");
                    exit(1);
                }
            }
        }
        "lies.log".into()
    }

    fn setup_log_file(&mut self) {
        let log_file = Self::get_log_file_target();
        if log_file.exists() {
            std::fs::copy(&log_file, format!("{}.old", log_file.to_str().unwrap())).unwrap();
        }
        self.log_stdout = Some(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(log_file)
                .unwrap(),
        );
    }

    fn get_stdout(&self) -> impl Write {
        MirrorStdout {
            stdout: io::stdout(),
            log_stdout: self.log_stdout.as_ref().map(|f| f.try_clone().unwrap()),
        }
    }

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
        self.get_stdout().is_terminal()
    }
}

impl Log for InternalLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let record_body = record.args().to_string();
        let timestamp = chrono::Local::now().format("%H:%M:%S");
        let message = format!(
            "[{}][{}] {}",
            timestamp,
            record.level(),
            self.color(record.level(), &record_body)
        );

        let mut stdout = self.get_stdout();
        stdout.write_all(message.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();

        if record.level() == Level::Error {
            panic!("Encountered error level log message: \"{}\"", record_body);
        }
    }

    fn flush(&self) {
        self.get_stdout().flush().unwrap();
    }
}
