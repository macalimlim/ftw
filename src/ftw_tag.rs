use crate::traits::ToGitTag;
use crate::type_alias::GitTag;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum FtwTag {
    Latest,
    Tagged { git_tag: GitTag },
}

const DEFAULT_TEMPLATE_TAG: &str = "v1.4.0";

impl FromStr for FtwTag {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latest" => Ok(FtwTag::default()),
            git_tag => Ok(FtwTag::Tagged {
                git_tag: git_tag.to_string(),
            }),
        }
    }
}

impl ToGitTag for FtwTag {
    fn to_git_tag(&self) -> GitTag {
        match self {
            FtwTag::Latest {} => DEFAULT_TEMPLATE_TAG,
            FtwTag::Tagged { git_tag } => git_tag,
        }
        .to_string()
    }
}

impl Display for FtwTag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            FtwTag::Latest {} => DEFAULT_TEMPLATE_TAG,
            FtwTag::Tagged { git_tag } => git_tag,
        };
        write!(f, "{message}")
    }
}

impl Default for FtwTag {
    fn default() -> Self {
        FtwTag::Latest {}
    }
}

#[cfg(test)]
mod ftw_tag_tests {
    use super::*;

    #[test]
    fn test_from_str() -> Result<(), ()> {
        assert_eq!(FtwTag::Latest {}, "latest".parse()?);
        assert_eq!(
            FtwTag::Tagged {
                git_tag: String::from("v1.1.0")
            },
            "v1.1.0".parse()?
        );
        Ok(())
    }

    #[test]
    fn test_to_git_tag() {
        assert_eq!(FtwTag::Latest.to_git_tag(), "v1.3.0");
        assert_eq!(
            FtwTag::Tagged {
                git_tag: String::from("v1.1.0")
            }
            .to_git_tag(),
            "v1.1.0"
        );
    }

    #[test]
    fn test_fmt() {
        let latest = FtwTag::Latest;
        let git_tag = String::from("v1.1.0");
        let tagged = FtwTag::Tagged { git_tag };
        assert_eq!(format!("{latest}"), "v1.3.0");
        assert_eq!(format!("{tagged}"), "v1.1.0");
    }

    #[test]
    fn test_default() {
        assert_eq!(FtwTag::default(), FtwTag::Latest);
    }
}
