use crate::ftw_error::FtwError;
use crate::ftw_success::FtwSuccess;

pub type GitUrl = String;
pub type GitTag = String;
pub type ProjectName = String;
pub type ClassName = String;
pub type CliArg = String;
pub type ExportArg = String;
pub type ExportName = String;
pub type LibExt = String;
pub type AppExt = String;
pub type LibPrefix = String;
pub type Message = String;
pub type StrTarget = String;
pub type FtwResult<'a> = Result<FtwSuccess<'a>, FtwError>;
