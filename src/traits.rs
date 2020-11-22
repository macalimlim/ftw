use crate::type_alias::GitUrl;

pub trait Processor {
    fn process(&self);
}

pub trait ToGitUrl {
    fn to_git_url(&self) -> GitUrl;
}
