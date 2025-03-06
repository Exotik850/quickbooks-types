#[derive(Debug, thiserror::Error)]
pub enum QBTypeError {
    #[cfg(feature = "builder")]
    #[error("Error validating QB Object in builder: {0}")]
    ValidationError(String),
    #[cfg(feature = "builder")]
    #[error("Uninitalized fields in QB Object in builder: {0}")]
    UninitializedField(String),
    #[error("QB Item could not be referenced!")]
    QBToRefError,
}
