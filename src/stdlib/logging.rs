use core::fmt;
// use lazy_static::lazy_static;
use log::{self, Level, LevelFilter, Log, Metadata, Record};

use crate::{println, stdlib::io::print};
// use crate::sync::SpinNoIrqLock as Mutex;

// static LOG_LOCK: Mutex<()> = Mutex::new(());

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    let log_level = match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn")  => LevelFilter::Warn,
        Some("info")  => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    };
    println!("Log level: {}", log_level);

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log_level);
}

/// Add escape sequence to print with color in Linux console
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\x1b[{}m{}\x1b[0m", $color_code as u8, $args)
    }};
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    unsafe {
        // thread unsafe
    }
    print(with_color!(args, color_code));
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
        print_in_color(
            format_args!(
                "[{:>5}] {}\n",
                record.level(),
                // crate::arch::cpu::id(),
                record.args()
            ),
            level_to_color_code(record.level()),
        );
    }
    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn  => 93,  // BrightYellow
        Level::Info  => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}