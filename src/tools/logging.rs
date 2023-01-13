use alloc::format;
// use lazy_static::lazy_static;
use log::{self, Level, LevelFilter, Log, Metadata, Record};

use crate::{
    println,
    tools::ansi::{Color, Colour},
};
// use crate::sync::SpinNoIrqLock as Mutex;

// static LOG_LOCK: Mutex<()> = Mutex::new(());

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    let log_level = match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    };
    println!("[kernel] Log level: {}", log_level);

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log_level);
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level = record.level();
        let color = level_color(level);
        println!("[{}] {}", level.dye(color), record.args());
    }
    fn flush(&self) {}
}

fn level_color(level: Level) -> Color {
    match level {
        Level::Error => Color::Red,
        Level::Warn => Color::YellowB,
        Level::Info => Color::Blue,
        Level::Debug => Color::Cyan,
        Level::Trace => Color::BlackB,
    }
}
