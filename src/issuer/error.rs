use thiserror::Error;

#[derive(Error, Debug)]
pub enum IssuerError {
    #[error("Invalid type: {0}")]
    InvalidType(String),
    #[error("Schema validation error: {0}")]
    SchemaValidationError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Signing error: {0}")]
    SigningError(String),
}
