use std::fmt;

#[derive(Debug)]
pub enum HolderError {
    StorageError(String),
    SerializationError(String),
    CredentialNotFound(String),
    ProofCreationError(String),
}

impl fmt::Display for HolderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HolderError::StorageError(msg) => write!(f, "Storage Error: {}", msg),
            HolderError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            HolderError::CredentialNotFound(id) => write!(f, "Credential Not Found: {}", id),
            HolderError::ProofCreationError(msg) => write!(f, "Proof Creation Error: {}", msg),
        }
    }
}

impl std::error::Error for HolderError {}
