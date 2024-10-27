use crate::SyncMutex;
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::OnceLock,
    time::SystemTime,
};

use colored::Colorize;
use log::{Metadata, Record};
use time::{format_description, OffsetDateTime};

pub use log;
pub use log::{debug, error, info, trace, warn, Level as LogLevel, LevelFilter as LogLevelFilter};

pub struct Logger {
    pub format_desc: Vec<format_description::FormatItem<'static>>,
    self_crate_name: &'static str,
    file_writer: Option<SyncMutex<BufWriter<File>>>,
}

// this is the callback that the static logger will use to log messages
// arguments: log level, message, message length
pub type StaticLoggerCallback = unsafe extern "C" fn(i32, *const u8, usize);

// logger for when the server is statically linked and not a dedicated application
pub struct StaticLogger {
    self_crate_name: &'static str,
    callback: Option<StaticLoggerCallback>,
}

const TIME_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]";
const LOG_BUFFER_CAPACITY: usize = 2048;

impl Logger {
    #[allow(clippy::missing_panics_doc)]
    pub fn instance(self_crate_name: &'static str, write_to_file: bool) -> &'static Self {
        static INSTANCE: OnceLock<Logger> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            format_desc: format_description::parse_borrowed::<2>(TIME_FORMAT).unwrap(),
            self_crate_name,
            file_writer: if write_to_file {
                let file = File::create(std::env::current_dir().unwrap().join(format!("{self_crate_name}.log")));

                if let Ok(file) = file {
                    Some(SyncMutex::new(BufWriter::with_capacity(LOG_BUFFER_CAPACITY, file)))
                } else {
                    eprintln!("failed to open log file for writing: {}", file.unwrap_err());
                    None
                }
            } else {
                None
            },
        })
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if metadata.target().starts_with(self.self_crate_name) {
            true
        } else {
            metadata.level() <= LogLevel::Warn
        }
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let now: OffsetDateTime = SystemTime::now().into();
        let formatted_time = now.format(&self.format_desc).unwrap();

        if let Some(file) = self.file_writer.as_ref() {
            let mut file = file.lock();
            if let Err(e) = writeln!(file, "[{formatted_time}] [{}] - {}", record.level(), record.args()) {
                eprintln!("Failed to write to the logfile: {e}");
            }
        }

        let (level, args) = match record.level() {
            LogLevel::Error => (record.level().to_string().bright_red(), record.args().to_string().bright_red()),
            LogLevel::Warn => (record.level().to_string().bright_yellow(), record.args().to_string().bright_yellow()),
            LogLevel::Info => (record.level().to_string().cyan(), record.args().to_string().cyan()),
            LogLevel::Debug => (record.level().to_string().white(), record.args().to_string().white()),
            LogLevel::Trace => (record.level().to_string().normal(), record.args().to_string().normal()),
        };

        if record.level() == LogLevel::Error {
            eprintln!("[{formatted_time}] [{level}] - {args}");
        } else {
            println!("[{formatted_time}] [{level}] - {args}");
        }
    }

    fn flush(&self) {
        self.file_writer.as_ref().map(|w| w.lock().flush());
    }
}

impl StaticLogger {
    #[allow(clippy::missing_panics_doc)]
    pub fn instance(self_crate_name: &'static str, callback: Option<StaticLoggerCallback>) -> &'static Self {
        static INSTANCE: OnceLock<StaticLogger> = OnceLock::new();
        INSTANCE.get_or_init(|| Self { self_crate_name, callback })
    }
}

impl log::Log for StaticLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if metadata.target().starts_with(self.self_crate_name) {
            true
        } else {
            metadata.level() <= LogLevel::Warn
        }
    }

    fn log(&self, record: &Record) {
        // if !self.enabled(record.metadata()) {
        //     return;
        // }

        if let Some(cb) = &self.callback {
            let message = format!("{}", record.args());
            let level = match record.level() {
                LogLevel::Error => 0,
                LogLevel::Warn => 1,
                LogLevel::Info => 2,
                LogLevel::Debug => 3,
                LogLevel::Trace => 4,
            };

            // safety: we trust that the caller supplied a valid callback
            unsafe {
                let message = format!("[{}] {}", record.metadata().target(), message);
                cb(level, message.as_ptr(), message.len());
            }
        }
    }
    fn flush(&self) {}
}
