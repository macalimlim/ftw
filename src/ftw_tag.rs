use crate::traits::ToGitTag;
use crate::type_alias::GitTag;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum FtwTag {
    Latest {},
    Tagged { tag: GitTag },
}

const DEFAULT_TEMPLATE_TAG: &str = "v1.0.0";

impl FromStr for FtwTag {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latest" => Ok(FtwTag::default()),
            tag => Ok(FtwTag::Tagged {
                tag: tag.to_string(),
            }),
        }
    }
}

impl ToGitTag for FtwTag {
    fn to_git_tag(&self) -> GitTag {
        match self {
            FtwTag::Latest {} => DEFAULT_TEMPLATE_TAG,
            FtwTag::Tagged { tag } => tag,
        }
        .to_string()
    }
}

impl Display for FtwTag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            FtwTag::Latest {} => "latest",
            FtwTag::Tagged { tag } => tag,
        };
        write!(f, "{}", message)
    }
}

impl Default for FtwTag {
    fn default() -> Self {
        FtwTag::Latest {}
    }
}
