use rakhas_logger::{LoggerConfig, LoggerHandle, stop_logger_gracefully};
use std::{path::PathBuf, process::ExitCode};

#[tokio::main]
async fn main() -> ExitCode {
    let config = LoggerConfig {
        log_path: PathBuf::from("app.log"),
        ..Default::default()
    };

    let logger = match LoggerHandle::new(config).await {
        Ok(logger) => logger,
        Err(e) => {
            eprintln!("Failed to create logger: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Log some messages
    if let Err(_) = logger.log("Application started".to_string()).await {
        return ExitCode::FAILURE;
    }

    if let Err(exit_code) = logger
        .err_log("Warning: something happened".to_string())
        .await
    {
        return ExitCode::FAILURE;
    }

    // Graceful shutdown
    stop_logger_gracefully(logger).await
}
