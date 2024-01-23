use std::{
    borrow::Borrow,
    env::{self, args},
    fs,
    io::{self, IsTerminal, Stdout, Write},
    path::{Path, PathBuf},
    process::exit,
};

use log::{Level, LevelFilter, Log};
use once_cell::sync::Lazy;


#[derive(Default)]
struct InternalLogger {
    log_stdout: Option<fs::File>,
}

static LOGGER: Lazy<InternalLogger> = Lazy::new(|| {
    let mut _self = InternalLogger::default();
    _self.setup_log_file();
    _self
});

pub fn init() {
    log::set_logger((*LOGGER).borrow()).unwrap();

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

    fn get_stdout(&self) -> (Stdout, Option<&std::fs::File>) {
        let stdout = io::stdout();
        let log_stdout = self.log_stdout.as_ref();
        (stdout, log_stdout)
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
        self.get_stdout().0.is_terminal()
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

        let mut stdout = self.get_stdout();
        stdout.0.write_all(headers.as_bytes()).unwrap();
        stdout.0.write_all(b" ").unwrap();
        stdout.0.write_all(colored.as_bytes()).unwrap();
        stdout.0.write_all(b"\n").unwrap();
        stdout.0.flush().unwrap();
        if let Some(mut log_stdout) = stdout.1 {
            log_stdout.write_all(headers.as_bytes()).unwrap();
            log_stdout.write_all(b" ").unwrap();
            log_stdout.write_all(record_body.as_bytes()).unwrap();
            log_stdout.write_all(b"\n").unwrap();
            log_stdout.flush().unwrap();
        }

        if record.level() == Level::Error {
            panic!("Encountered error level log message: \"{}\"", record_body);
        }
    }

    fn flush(&self) {
        let mut stdout = self.get_stdout();
        stdout.0.flush().unwrap();
        if let Some(mut log_stdout) = stdout.1 {
            log_stdout.flush().unwrap();
        }
    }
}
