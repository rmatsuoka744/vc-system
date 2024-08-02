use thiserror::Error;

#[derive(Error, Debug)]
pub enum IssuerError {
    #[error("Invalid context: {0}")]
    InvalidContext(String),
    #[error("Invalid type: {0}")]
    InvalidType(String),
    #[error("Invalid issuer: {0}")]
    InvalidIssuer(String),
    #[error("Invalid issuance date: {0}")]
    InvalidIssuanceDate(String),
    #[error("Invalid credential subject: {0}")]
    InvalidCredentialSubject(String),
    #[error("Credential creation error: {0}")]
    CredentialCreationError(String),
    #[error("Signing error: {0}")]
    SigningError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Schema validation error: {0}")]
    SchemaValidationError(String),
}
