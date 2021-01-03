use thiserror::Error;

#[derive(Debug, Error)]
pub enum FtwError {
    #[error("Error: {0}")]
    Error(#[from] std::io::Error),
    #[error("Invalid project")]
    InvalidProject,
    #[error("Liquid error: {0}")]
    LiquidError(#[from] liquid_core::Error),
    #[error("UTF-8 conversion error")]
    Utf8ConversionError,
}
