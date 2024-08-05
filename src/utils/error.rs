use std::fmt;

#[derive(Debug)]
pub enum UtilsError {
    JsonSerializationError(String),
    SignatureError(String),
}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for UtilsError {}
