use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Notification error: {0}")]
    NotificationError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}
