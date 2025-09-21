use tokio::sync::oneshot;

#[derive(Debug)]
pub enum LogCommand {
    Write(String),
    Stop(oneshot::Sender<()>),
}

impl LogCommand {
    pub fn message(&self) -> String {
        if let LogCommand::Write(message) = self {
            message.clone()
        } else {
            String::new()
        }
    }
}
