use crate::traits::ToGitUrl;
use crate::type_alias::GitUrl;
use std::str::FromStr;

pub enum FtwTemplate {
    Default,
    Custom { git_url: GitUrl },
}

impl FromStr for FtwTemplate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(FtwTemplate::Default),
            git_url => Ok(FtwTemplate::Custom {
                git_url: git_url.to_string(),
            }),
        }
    }
}

impl ToGitUrl for FtwTemplate {
    fn to_git_url(&self) -> GitUrl {
        match self {
            FtwTemplate::Default => "https://github.com/godot-rust/godot-rust-template",
            FtwTemplate::Custom { git_url } => git_url,
        }
        .to_string()
    }
}
