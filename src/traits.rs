use crate::ftw_error::FtwError;
use crate::type_alias::{
    AppExt, CliArg, ExportArg, ExportName, FtwResult, GitTag, GitUrl, LibExt, LibPrefix, Message,
};

pub trait Processor {
    /// # Errors
    ///
    /// Will return `Err` if an error happened in the implementation
    fn process(&self) -> FtwResult;
}

pub trait Runner {
    /// # Errors
    ///
    /// Will return `Err` if an error happened in the implementation
    fn run(&mut self) -> Result<(), FtwError>;
}

pub trait Compiler {
    /// # Errors
    ///
    /// Will return `Err` if an error happened in the implementation
    fn clean(&self) -> Result<(), FtwError>;
    /// # Errors
    ///
    /// Will return `Err` if an error happened in the implementation
    fn build(&self) -> Result<(), FtwError>;
    /// # Errors
    ///
    /// Will return `Err` if an error happened in the implementation
    fn export(&self) -> Result<(), FtwError>;
}

pub trait ToGitUrl {
    fn to_git_url(&self) -> GitUrl;
}

pub trait ToGitTag {
    fn to_git_tag(&self) -> GitTag;
}

pub trait ToCliArg {
    fn to_cli_arg(&self) -> CliArg;
}

pub trait ToExportArg {
    fn to_export_arg(&self) -> ExportArg;
}

pub trait ToExportName {
    fn to_export_name(&self) -> ExportName;
}

pub trait ToLibExt {
    fn to_lib_ext(&self) -> LibExt;
}

pub trait ToAppExt {
    fn to_app_ext(&self) -> AppExt;
}

pub trait ToLibPrefix {
    fn to_lib_prefix(&self) -> LibPrefix;
}

pub trait ToMessage {
    fn to_message(&self) -> Message;
}
