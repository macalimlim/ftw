use crate::ftw_error::FtwError;
use crate::type_alias::{CliArg, GitUrl, LibExt, LibPrefix};

pub trait Processor {
    fn process(&self) -> Result<(), FtwError>;
}

pub trait ToGitUrl {
    fn to_git_url(&self) -> GitUrl;
}

pub trait ToCliArg {
    fn to_cli_arg(&self) -> CliArg;
}

pub trait ToLibExt {
    fn to_lib_ext(&self) -> LibExt;
}

pub trait ToLibPrefix {
    fn to_lib_prefix(&self) -> LibPrefix;
}
