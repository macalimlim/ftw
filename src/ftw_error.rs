use thiserror::Error;

#[derive(Debug, Error)]
pub enum FtwError {
    #[error("Error: {0}")]
    Error(#[from] std::io::Error),
    #[error("Invalid project")]
    InvalidProject,
    #[error("Liquid error: {0}")]
    LiquidError(#[from] liquid_core::Error),
    #[error("Walkdir error")]
    WalkdirError(#[from] walkdir::Error),
    #[error("Unsupported target")]
    UnsupportedTarget,
    #[error("FsExtra error: {0}")]
    FsExtraError(#[from] fs_extra::error::Error),
    #[error("Unknown build type")]
    UnknownBuildType,
    #[error("Cargo edit error: {0}")]
    CargoEditError(#[from] cargo_edit::Error),
    #[error("Path error")]
    PathError,
    #[error("String conversion error")]
    StringConversionError,
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}
