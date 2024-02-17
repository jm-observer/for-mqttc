use crate::data::AppEvent;
use crossbeam_channel::Sender;
use flexi_logger::writers::LogWriter;
use flexi_logger::DeferredNow;
use log::{Level, Record};

pub struct CustomWriter(pub Sender<AppEvent>);

impl LogWriter for CustomWriter {
    fn write(&self, _now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        let level = record.level();
        if level <= self.max_log_level() {
            let _msg = if level <= Level::Warn {
                format!("{}: {}", level, record.args())
            } else {
                record.args().to_string()
            };
        }
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn max_log_level(&self) -> log::LevelFilter {
        log::LevelFilter::Info
    }
}
