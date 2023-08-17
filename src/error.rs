#[cfg(feature = "builder")]
#[derive(Debug, thiserror::Error)]
pub enum QBError {
    #[error("Error validating QB Object in builder: {0}")]
    ValidationError(String),
    #[error("Uninitalized fields in QB Object in builder: {0}")]
    UninitializedField(String),
}
