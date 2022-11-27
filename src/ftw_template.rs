use crate::type_alias::{GitTag, GitUrl};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FtwTemplate {
    Default {
        git_url: GitUrl,
        tag: Option<GitTag>,
    },
    Custom {
        git_url: GitUrl,
        tag: Option<GitTag>,
    },
}

const DEFAULT_TEMPLATE_URL: &str = "https://github.com/macalimlim/godot-rust-template";
const DEFAULT_TEMPLATE_TAG: &str = "v1.0.0";

impl FromStr for FtwTemplate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(FtwTemplate::default()),
            git_url => Ok(FtwTemplate::Custom {
                git_url: git_url.to_string(),
                tag: None,
            }),
        }
    }
}

impl Display for FtwTemplate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            FtwTemplate::Default { git_url: _, tag: _ } => "default",
            FtwTemplate::Custom { git_url: _, tag: _ } => "custom",
        };
        write!(f, "{}", message)
    }
}

impl Default for FtwTemplate {
    fn default() -> Self {
        FtwTemplate::Default {
            git_url: DEFAULT_TEMPLATE_URL.to_string(),
            tag: Some(DEFAULT_TEMPLATE_TAG.to_string()),
        }
    }
}

#[cfg(test)]
mod ftw_template_tests {
    use super::*;
    use proptest::prelude::{prop_assert, prop_assert_eq, prop_assume, proptest};

    const CUSTOM_TEMPLATE: &str = "/path/to/custom/template";

    #[test]
    fn test_from_str() -> Result<(), ()> {
        let custom_template = CUSTOM_TEMPLATE.to_string();
        assert_eq!(FtwTemplate::default(), "default".parse()?);
        assert_eq!(
            FtwTemplate::Custom {
                git_url: custom_template.clone(),
                tag: None
            },
            custom_template.parse()?
        );
        Ok(())
    }

    #[test]
    fn test_to_git_url() {
        if let FtwTemplate::Default { git_url, tag } = FtwTemplate::default() {
            assert_eq!(git_url, DEFAULT_TEMPLATE_URL);
            assert_eq!(tag.unwrap_or_default(), DEFAULT_TEMPLATE_TAG);
        }
        let custom_template = CUSTOM_TEMPLATE.to_string();
        let tpl = FtwTemplate::Custom {
            git_url: custom_template.clone(),
            tag: None,
        };
        if let FtwTemplate::Custom { git_url, tag } = tpl {
            assert_eq!(git_url, custom_template);
            assert_eq!(tag, None);
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!(
            format!("{}", "default"),
            format!("{}", FtwTemplate::default())
        );
        let custom_template = CUSTOM_TEMPLATE.to_string();
        assert_eq!(
            format!("{}", "custom"),
            format!(
                "{}",
                FtwTemplate::Custom {
                    git_url: custom_template,
                    tag: None
                }
            )
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            FtwTemplate::default(),
            FtwTemplate::Default {
                git_url: DEFAULT_TEMPLATE_URL.to_string(),
                tag: Some(DEFAULT_TEMPLATE_TAG.to_string())
            }
        );
    }

    proptest! {
        #[test]
        fn test_from_str_custom(template_input in "\\PC*") {
            prop_assume!(template_input != "default");
            prop_assert!(template_input.parse::<FtwTemplate>().is_ok());
            prop_assert_eq!(FtwTemplate::Custom{git_url: template_input.to_string(), tag: None}, template_input.parse::<FtwTemplate>().unwrap());
        }
    }
}
