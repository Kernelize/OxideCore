use core::arch;

use crate::{cpu, println};
use log::{self, Level, LevelFilter, Log, Metadata, Record};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let color_code = level_to_color_code(level);
            let args = record.args();
            println!(
                "[{}][{}] - {}",
                format_args!("\x1b[{}m{}\x1b[0m", color_code, level,),
                cpu::id(),
                format_args!("\x1b[{}m{}\x1b[0m", color_code, args,),
            );
        }
    }
    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("trace") => LevelFilter::Trace,
        Some("debug") => LevelFilter::Debug,
        Some("info") => LevelFilter::Info,
        Some("warn") => LevelFilter::Warn,
        Some("error") => LevelFilter::Error,
        Some("off") => LevelFilter::Off,
        _ => LevelFilter::Trace,
    });
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
