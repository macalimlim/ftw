use crate::ftw_error::FtwError;
use crate::traits::{ToCliArg, ToExportArg};
use crate::type_alias::{CliArg, ExportArg};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum FtwBuildType {
    Debug,
    Release,
}

impl ToCliArg for FtwBuildType {
    fn to_cli_arg(&self) -> CliArg {
        match self {
            FtwBuildType::Debug => "",
            FtwBuildType::Release => "--release",
        }
        .to_string()
    }
}

impl ToExportArg for FtwBuildType {
    fn to_export_arg(&self) -> ExportArg {
        match self {
            FtwBuildType::Debug => "--export-debug",
            FtwBuildType::Release => "--export",
        }
        .to_string()
    }
}

impl Display for FtwBuildType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let build_type: &str = match self {
            FtwBuildType::Debug => "debug",
            FtwBuildType::Release => "release",
        };
        write!(f, "{}", build_type)
    }
}

impl FromStr for FtwBuildType {
    type Err = FtwError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "debug" => Ok(FtwBuildType::Debug),
            "release" => Ok(FtwBuildType::Release),
            _ => Err(FtwError::UnknownBuildType),
        }
    }
}

#[cfg(test)]
mod ftw_build_type_tests {
    use super::*;
    use proptest::prelude::{prop_assert, prop_assume, proptest};

    #[test]
    fn test_to_cli_arg() {
        assert_eq!("", FtwBuildType::Debug.to_cli_arg());
        assert_eq!("--release", FtwBuildType::Release.to_cli_arg());
    }

    #[test]
    fn test_to_export_arg() {
        assert_eq!("--export-debug", FtwBuildType::Debug.to_export_arg());
        assert_eq!("--export", FtwBuildType::Release.to_export_arg());
    }

    #[test]
    fn test_fmt() {
        assert_eq!("debug", format!("{}", FtwBuildType::Debug));
        assert_eq!("release", format!("{}", FtwBuildType::Release));
    }

    #[test]
    fn test_from_str() -> Result<(), FtwError> {
        assert_eq!(FtwBuildType::Debug, "debug".parse()?);
        assert_eq!(FtwBuildType::Release, "release".parse()?);
        Ok(())
    }

    proptest! {
        #[test]
        fn test_from_str_error(build_type_input in "\\PC*") {
            prop_assume!(build_type_input != "debug");
            prop_assume!(build_type_input != "release");
            prop_assert!(build_type_input.parse::<FtwBuildType>().is_err());
            if let FtwError::UnknownBuildType = build_type_input.parse::<FtwBuildType>().unwrap_err() {
                 prop_assert!(true);
            }
        }
    }
}
