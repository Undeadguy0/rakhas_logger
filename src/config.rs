use chrono_tz::Tz;
use std::path::PathBuf;
use std::time::Duration;

pub const BATCH_SIZE: usize = 1024;
pub const DELAY_TO_FORCE_DROP: Duration = Duration::from_secs(5);
pub const TIME_BEFORE_COUNTS_DEAD: Duration = Duration::from_secs(10);
pub const MOSCOW_ZONE: Tz = chrono_tz::Europe::Moscow;

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub log_path: PathBuf,
    pub timezone: Tz,
    pub batch_size: usize,
    pub flush_interval: Duration,
    pub timeout: Duration,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_path: PathBuf::from("app.log"),
            timezone: MOSCOW_ZONE,
            batch_size: BATCH_SIZE,
            flush_interval: DELAY_TO_FORCE_DROP,
            timeout: TIME_BEFORE_COUNTS_DEAD,
        }
    }
}
