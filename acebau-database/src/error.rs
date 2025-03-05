use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Cannot serialize data: {0}")]
    SerializeError(Box<dyn std::error::Error>),
    #[error("Cannot deserialize data: {0}")]
    DeserializeError(Box<dyn std::error::Error>),
    #[error("Internal error: {0}")]
    InternalError(Box<dyn std::error::Error>),
}
