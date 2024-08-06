use thiserror::Error;
use crate::utils::error::UtilsError;

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
    //#[error("JWT creation error: {0}")]
    //JwtCreationError(String),
    //#[error("Invalid SD-JWT format")]
    //InvalidSdJwtFormat,
    #[error("Utils error: {0}")]
    UtilsError(#[from] UtilsError),
}

impl From<String> for IssuerError {
    fn from(error: String) -> Self {
        IssuerError::SigningError(error)
    }
}
