use crate::ftw_error::FtwError;
use crate::traits::{ToCliArg, ToExportArg};
use crate::type_alias::{CliArg, ExportArg};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
