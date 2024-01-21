use std::{
    env::{self, args},
    io::{self, IsTerminal, Stdout, Write},
    process::exit,
};

use log::{Level, LevelFilter, Log};

struct InternalLogger;

pub fn init() {
    static LOGGER: InternalLogger = InternalLogger;
    log::set_logger(&LOGGER).unwrap();

    let mut highest_log_level = log::LevelFilter::Info;
    if let Ok(value) = env::var("LIES_LOG_LEVEL") {
        if let Ok(parsed) = value.parse() {
            highest_log_level = parsed;
        } else {
            match value.to_lowercase().as_str() {
                "error" => highest_log_level = LevelFilter::Error,
                "warn" => highest_log_level = LevelFilter::Warn,
                "info" => highest_log_level = LevelFilter::Info,
                "debug" => highest_log_level = LevelFilter::Debug,
                "trace" => highest_log_level = LevelFilter::Trace,
                _ => {
                    eprintln!("Invalid value for LIES_LOG_LEVEL");
                    exit(1);
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
}

impl InternalLogger {
    fn get_stdout(&self) -> Stdout {
        io::stdout()
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
    }

    fn flush(&self) {
        self.get_stdout().flush().unwrap();
    }
}
