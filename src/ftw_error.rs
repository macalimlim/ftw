use crate::traits::ToMessage;
use crate::type_alias::Message;
use colored::{ColoredString, Colorize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FtwError {
    #[error("{0}")]
    Error(#[from] std::io::Error),
    #[error("Invalid project")]
    InvalidProject,
    #[error("{0}")]
    LiquidError(#[from] liquid_core::Error),
    #[error("Walkdir error")]
    WalkdirError(#[from] walkdir::Error),
    #[error("Unsupported target")]
    UnsupportedTarget,
    #[error("{0}")]
    FsExtraError(#[from] fs_extra::error::Error),
    #[error("Unknown build type")]
    UnknownBuildType,
    #[error("{0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Missing package name error")]
    MissingPackageNameError,
    #[error("Path error")]
    PathError,
    #[error("String conversion error")]
    StringConversionError,
    #[error("{0}")]
    RegexError(#[from] regex::Error),
    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
}

impl FtwError {
    const THUMBS_DOWN: &'static str = "\u{f165}";

    fn get_styled_error() -> ColoredString {
        "ERROR:".bold().red()
    }
}

#[rustfmt::skip::macros(format)]
impl ToMessage for FtwError {
    fn to_message(&self) -> Message {
        format!("{} {} {}", FtwError::THUMBS_DOWN, FtwError::get_styled_error(), self.to_string().split('\n').next().unwrap_or("Unknown error"))
    }
}

#[cfg(test)]
mod ftw_error_tests {
    use super::*;

    #[test]
    fn test_to_message() {
        let io_error = FtwError::Error(std::io::Error::new(std::io::ErrorKind::Other, "IO error"));
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "IO error"
            ),
            io_error.to_message()
        );
        //
        let invalid_project_error = FtwError::InvalidProject;
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "Invalid project"
            ),
            invalid_project_error.to_message()
        );
        //
        let liquid_error = FtwError::LiquidError(liquid_core::Error::with_msg("Liquid error"));
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "liquid: Liquid error"
            ),
            liquid_error.to_message()
        );
        //
        // TODO: walkdir error
        //
        let unsupported_target_error = FtwError::UnsupportedTarget;
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "Unsupported target"
            ),
            unsupported_target_error.to_message()
        );
        //
        let fs_extra_error = FtwError::FsExtraError(fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::Other,
            "Fs extra error",
        ));
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "Fs extra error"
            ),
            fs_extra_error.to_message()
        );
        //
        let unknown_build_type_error = FtwError::UnknownBuildType;
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "Unknown build type"
            ),
            unknown_build_type_error.to_message()
        );
        //
        // TODO: cargo edit error
        //
        let path_error = FtwError::PathError;
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "Path error"
            ),
            path_error.to_message()
        );
        //
        let string_conversion_error = FtwError::StringConversionError;
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "String conversion error"
            ),
            string_conversion_error.to_message()
        );
        //
        let regex_error = FtwError::RegexError(regex::Error::Syntax("Syntax error".to_string()));
        assert_eq!(
            format!(
                "{} {} {}",
                FtwError::THUMBS_DOWN,
                FtwError::get_styled_error(),
                "Syntax error"
            ),
            regex_error.to_message()
        );
        //
        // TODO: anyhow error
    }
}
