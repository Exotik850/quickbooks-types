#[derive(Debug, thiserror::Error)]
pub enum QBTypeError {
    #[cfg(feature = "builder")]
    #[error("Error validating QB Object in builder: {0}")]
    ValidationError(String),
    #[cfg(feature = "builder")]
    #[error("Missing field in QB Object: {0}")]
    MissingField(&'static str),
    #[error("QB Item could not be referenced!")]
    QBToRefError,
}
