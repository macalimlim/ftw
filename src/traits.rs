use crate::ftw_error::FtwError;
use crate::type_alias::GitUrl;

pub trait Processor {
    fn process(&self) -> Result<(), FtwError>;
}

pub trait ToGitUrl {
    fn to_git_url(&self) -> GitUrl;
}
