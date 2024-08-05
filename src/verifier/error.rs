use thiserror::Error;
use crate::utils::error::UtilsError;

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Missing proof")]
    MissingProof,
    #[error("Untrusted issuer")]
    UntrustedIssuer,
    #[error("Invalid credential format")]
    InvalidCredentialFormat,
    #[error("Invalid base64 encoding")]
    InvalidBase64Encoding,
    #[error("Invalid JSON payload")]
    InvalidJsonPayload,
    #[error("Missing SD-ALG claim")]
    MissingSdAlgClaim,
    #[error("Signature verification failed: {0}")]
    SignatureVerificationFailed(String),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Utils error: {0}")]
    UtilsError(#[from] UtilsError),
}

impl From<String> for VerifierError {
    fn from(error: String) -> Self {
        VerifierError::InternalError(error)
    }
}
