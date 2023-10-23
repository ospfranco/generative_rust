use colored::Colorize;
use log::{Level, Metadata, Record};

pub struct SimpleLogger {
    pub enabled: bool,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info && self.enabled
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} - {}",
                record.level().to_string().yellow(),
                record.args().to_string().yellow()
            );
        }
    }

    fn flush(&self) {}
}
