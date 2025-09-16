use std::sync::OnceLock;

use colored::Colorize;
use log::{Level, LevelFilter};

static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug)]
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool
    {
        return true;
    }

    fn log(&self, record: &log::Record)
    {
        match record.level()
        {
            Level::Error =>
            {
                eprint!(
                    "{} {}",
                    "error:".red().bold(),
                    record.args()
                );
            }
            Level::Warn =>
            {
                eprint!(
                    "{} {}",
                    "warn:".yellow().bold(),
                    record.args()
                );
            }
            _ =>
            {
                eprint!("{}", record.args());
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    LOGGER.set(Logger { }).unwrap();

    log::set_logger(LOGGER.get().unwrap()).unwrap();
    log::set_max_level(LevelFilter::max());
}
