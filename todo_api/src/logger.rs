/* 
 *
 * Heavily inspired by `pretty_env_logger` https://crates.io/crates/pretty_env_logger/
 * 
 */

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use colored::*;
use log::{Metadata, Record, Log, LevelFilter, SetLoggerError, Level};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LoggerLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
    None,
}

impl From<LoggerLevel> for Option<Level> {
    fn from(value: LoggerLevel) -> Self {
        match value {
            LoggerLevel::Debug => Some(Level::Debug),
            LoggerLevel::Error => Some(Level::Error),
            LoggerLevel::Info => Some(Level::Info),
            LoggerLevel::Trace => Some(Level::Trace),
            LoggerLevel::Warning => Some(Level::Warn),
            LoggerLevel::None => None,
        }
    }
}

pub struct Logger {
    log_level: Level,
}

impl Logger {
    pub fn init(){
        Logger::try_init().expect("Failed to set logger");
    }

    pub fn try_init() -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::<Logger>::default())?;
        log::set_max_level(LevelFilter::Info);

        Ok(())
    }

    pub fn init_with_level(logger_level: LoggerLevel){
        Logger::try_init_with_level(logger_level).expect("Failed to set logger with level");
    }

    pub fn try_init_with_level(logger_level: LoggerLevel) -> Result<(), SetLoggerError> {
        if let Some(log_level) = logger_level.into() {
            log::set_boxed_logger(Box::new(Logger { log_level }))?;
            log::set_max_level(log_level.to_level_filter());
        }

        Ok(())
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger  {
            log_level: Level::Info,
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let target = split_target(record.target());
            let max_width = max_target_width(target);

            let level = colored_level(record.level());

            let target = Padded {
                value: target.bold(),
                width: max_width,
            };

            println!("{} {} > {}", level, target, record.args());
        }
    }

    fn flush(&self) {}
}

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn split_target(target: &str) -> &str {
    match target.split_once("::") {
        Some((module, _)) => module,
        None => target,
    }
}

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

fn colored_level(level: Level) -> ColoredString {
    match level {
        Level::Trace => "TRACE".magenta(),
        Level::Debug => "DEBUG".blue(),
        Level::Info =>  "INFO ".green(),
        Level::Warn =>  "WARN ".yellow(),
        Level::Error => "ERROR".red(),
    }
}