use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationError {
    #[allow(dead_code)]
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[allow(dead_code)]
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[allow(dead_code)]
    #[error("Notification error: {0}")]
    NotificationError(String),
    #[allow(dead_code)]
    #[error("Unknown error: {0}")]
    Unknown(String),
}
