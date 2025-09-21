use crate::config::LoggerConfig;
use crate::core::Logger;
use crate::core::command::LogCommand;
use crate::error::LoggerError;
use crate::formatter::format_log_message;

use colorize::AnsiColor;
use std::process::ExitCode;

use tokio::{
    sync::{
        mpsc::{Sender, channel},
        oneshot,
    },
    time::timeout,
};

pub struct LoggerHandle {
    logger_send_channel: Sender<LogCommand>,
    stop_sender: oneshot::Sender<()>,
    stop_receiver: oneshot::Receiver<()>,
    config: LoggerConfig,
}

impl LoggerHandle {
    pub async fn new(config: LoggerConfig) -> Result<Self, LoggerError> {
        let (logger_send_channel, logger_receive_channel) = channel(1024);
        let (stop_sender, stop_receiver) = oneshot::channel();

        let mut logger = Logger::new(config.clone(), logger_receive_channel).await?;

        tokio::spawn(async move {
            if let Err(e) = logger.init().await {
                eprintln!("Ошибка старта журналера: {}", e);
            }
        });

        Ok(Self {
            logger_send_channel,
            stop_sender,
            stop_receiver,
            config,
        })
    }

    pub async fn log(&self, message: String) -> Result<(), LoggerError> {
        let formatted = format_log_message(&message, &self.config.timezone, false).await;

        self.logger_send_channel
            .send(LogCommand::Write(formatted))
            .await
            .map_err(|e| LoggerError::Channel(e.to_string()))?;

        Ok(())
    }

    pub async fn err_log(&self, message: String) -> Result<(), LoggerError> {
        let formatted = format_log_message(&message, &self.config.timezone, true).await;

        self.logger_send_channel
            .send(LogCommand::Write(formatted))
            .await
            .map_err(|e| LoggerError::Channel(e.to_string()))?;

        Ok(())
    }

    pub async fn stop(self) -> Result<(), LoggerError> {
        let stop_message = format_log_message(
            "Работа журналера завершена успешно",
            &self.config.timezone,
            false,
        )
        .await;

        self.logger_send_channel
            .send(LogCommand::Write(stop_message))
            .await
            .map_err(|e| LoggerError::Channel(e.to_string()))?;

        self.logger_send_channel
            .send(LogCommand::Stop(self.stop_sender))
            .await
            .map_err(|e| LoggerError::Channel(e.to_string()))?;

        timeout(self.config.timeout, self.stop_receiver)
            .await
            .map_err(|_| LoggerError::Timeout)?
            .map_err(|_| LoggerError::Channel("Журналер не ответил на остановку".to_string()))?;

        Ok(())
    }

    pub async fn try_log(&self, message: String, is_error: bool) -> Result<(), ExitCode> {
        let result = if is_error {
            self.err_log(message).await
        } else {
            self.log(message).await
        };

        result.map_err(|e| {
            eprintln!("{}", format!("Ошибка логирования: {}", e).bold().red());
            ExitCode::FAILURE
        })
    }
}

pub async fn stop_logger_gracefully(handler: LoggerHandle) -> ExitCode {
    handler.stop().await.map_or_else(
        |e| {
            eprintln!(
                "{}",
                format!("Ошибка остановки журналера: {}", e).bold().red()
            );
            ExitCode::FAILURE
        },
        |_| ExitCode::SUCCESS,
    )
}
