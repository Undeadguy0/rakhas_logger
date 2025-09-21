//! Async logging library with batch processing and graceful shutdown

pub mod config;
pub mod core;
pub mod error;
pub mod formatter;
pub mod handlers;

// Re-export main types for convenience
pub use config::{LoggerConfig, MOSCOW_ZONE};
pub use error::LoggerError;
pub use handlers::logger_handler::{LoggerHandle, stop_logger_gracefully};

// For backward compatibility
pub use formatter::format_log_message as format_for_log;
