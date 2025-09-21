use crate::config::LoggerConfig;
use crate::core::command::LogCommand;
use crate::error::LoggerError;
use crate::formatter::format_log_message;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, select, sync::mpsc::Receiver, time::sleep};

pub struct Logger {
    config: LoggerConfig,
    input_channel: Receiver<LogCommand>,
}

impl Logger {
    pub async fn new(
        config: LoggerConfig,
        input_channel: Receiver<LogCommand>,
    ) -> Result<Self, LoggerError> {
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.log_path)
            .await?;

        let welcome_message =
            format_log_message("Подключение к журналу успешно", &config.timezone, false).await;

        file.write_all(welcome_message.as_bytes()).await?;

        Ok(Self {
            config,
            input_channel,
        })
    }

    pub async fn init(&mut self) -> Result<(), LoggerError> {
        let mut buffer: Vec<String> = Vec::with_capacity(self.config.batch_size);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.config.log_path)
            .await?;

        loop {
            select! {
                // Обработка сообщений из канала
                Some(message) = self.input_channel.recv() => {
                    match message {
                        LogCommand::Write(text) => {
                            buffer.push(text);

                            if buffer.len() >= self.config.batch_size {
                                let full_batch = buffer.concat();
                                file.write_all(full_batch.as_bytes()).await?;
                                buffer.clear();
                            }
                        }
                        LogCommand::Stop(callback) => {
                            if !buffer.is_empty() {
                                let full_batch = buffer.concat();
                                file.write_all(full_batch.as_bytes()).await?;
                                buffer.clear();
                            }

                            callback.send(()).map_err(|_| {
                                LoggerError::Channel("Не удалось отправить подтверждение остановки".to_string())
                            })?;
                            return Ok(());
                        }
                    }
                }
                // Принудительный сброс по таймеру
                _ = sleep(self.config.flush_interval) => {
                    if !buffer.is_empty() {
                        let full_batch = buffer.concat();
                        file.write_all(full_batch.as_bytes()).await?;
                        buffer.clear();
                    }
                }
            }
        }
    }
}
