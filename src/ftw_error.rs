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
        let thumbs_down = FtwError::THUMBS_DOWN;
        let styled_error = FtwError::get_styled_error();
        let error_string = self.to_string();
        let description = error_string.split('\n').next().unwrap_or("Unknown error");
        format!("{thumbs_down} {styled_error} {description}")
    }
}

#[cfg(test)]
mod ftw_error_tests {
    use super::*;

    #[test]
    fn test_to_message() {
        let error_description = "IO error";
        let io_error_message = FtwError::Error(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_description,
        ))
        .to_message();
        let thumbs_down = FtwError::THUMBS_DOWN;
        let styled_error = FtwError::get_styled_error();
        assert_eq!(
            format!("{thumbs_down} {styled_error} {error_description}"),
            io_error_message
        );
        //
        let invalid_project_error_message = FtwError::InvalidProject.to_message();
        assert_eq!(
            format!("{thumbs_down} {styled_error} Invalid project"),
            invalid_project_error_message
        );
        //
        let error_description = "Liquid error";
        let liquid_error_message =
            FtwError::LiquidError(liquid_core::Error::with_msg(error_description)).to_message();
        assert_eq!(
            format!("{thumbs_down} {styled_error} liquid: {error_description}"),
            liquid_error_message
        );
        //
        // TODO: walkdir error
        //
        let unsupported_target_error = FtwError::UnsupportedTarget;
        assert_eq!(
            format!("{thumbs_down} {styled_error} Unsupported target"),
            unsupported_target_error.to_message()
        );
        //
        let error_description = "Fs extra error";
        let fs_extra_error_message = FtwError::FsExtraError(fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::Other,
            error_description,
        ))
        .to_message();
        assert_eq!(
            format!("{thumbs_down} {styled_error} {error_description}"),
            fs_extra_error_message
        );
        //
        let unknown_build_type_error_message = FtwError::UnknownBuildType.to_message();
        assert_eq!(
            format!("{thumbs_down} {styled_error} Unknown build type"),
            unknown_build_type_error_message
        );
        //
        // TODO: cargo edit error
        //
        let path_error_message = FtwError::PathError.to_message();
        assert_eq!(
            format!("{thumbs_down} {styled_error} Path error"),
            path_error_message
        );
        //
        let string_conversion_error_message = FtwError::StringConversionError.to_message();
        assert_eq!(
            format!("{thumbs_down} {styled_error} String conversion error"),
            string_conversion_error_message
        );
        //
        let error_description = "Syntax error";
        let regex_error_message =
            FtwError::RegexError(regex::Error::Syntax(error_description.to_string())).to_message();
        assert_eq!(
            format!("{thumbs_down} {styled_error} {error_description}"),
            regex_error_message
        );
        //
        // TODO: anyhow error
    }
}
