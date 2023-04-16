#[derive(Debug, thiserror::Error)]
pub enum DataAccessError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Not found")]
    NotFoundError,
}

impl From<diesel::result::Error> for DataAccessError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::NotFoundError,
            err => Self::ExecutionError(err.to_string()),
        }
    }
}
