#[derive(Debug, thiserror::Error)]
pub enum DataAccessError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
}
