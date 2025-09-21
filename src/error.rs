use std::process::ExitCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Channel error: {0}")]
    Channel(String),

    #[error("Initialization error: {0}")]
    Init(String),

    #[error("Timeout error")]
    Timeout,
}

impl LoggerError {
    pub fn to_exit_code(&self) -> ExitCode {
        ExitCode::FAILURE
    }
}
