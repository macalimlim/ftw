use crate::traits::ToGitUrl;
use crate::type_alias::GitUrl;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum FtwTemplate {
    Default,
    Custom { git_url: GitUrl },
}

const DEFAULT_TEMPLATE_URL: &str = "https://github.com/godot-rust/godot-rust-template";

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
            FtwTemplate::Default => DEFAULT_TEMPLATE_URL,
            FtwTemplate::Custom { git_url } => git_url,
        }
        .to_string()
    }
}

impl Display for FtwTemplate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message: String = match self {
            FtwTemplate::Default => format!("the default ({})", DEFAULT_TEMPLATE_URL),
            FtwTemplate::Custom { git_url } => format!("a custom ({})", git_url),
        };
        write!(f, "{}", message)
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
        assert_eq!(FtwTemplate::Default, "default".parse()?);
        assert_eq!(
            FtwTemplate::Custom {
                git_url: custom_template.clone(),
            },
            custom_template.parse()?
        );
        Ok(())
    }

    #[test]
    fn test_to_git_url() {
        let custom_template = CUSTOM_TEMPLATE.to_string();
        assert_eq!(FtwTemplate::Default.to_git_url(), DEFAULT_TEMPLATE_URL);
        assert_eq!(
            FtwTemplate::Custom {
                git_url: custom_template.clone()
            }
            .to_git_url(),
            custom_template
        );
    }

    #[test]
    fn test_fmt() {
        let custom_template = CUSTOM_TEMPLATE.to_string();
        assert_eq!(
            format!("the default ({})", DEFAULT_TEMPLATE_URL),
            format!("{}", FtwTemplate::Default)
        );
        assert_eq!(
            format!("a custom ({})", custom_template),
            format!(
                "{}",
                FtwTemplate::Custom {
                    git_url: custom_template
                }
            )
        );
    }

    proptest! {
        #[test]
        fn test_from_str_custom(template_input in "\\PC*") {
            prop_assume!(template_input != "default");
            prop_assert!(template_input.parse::<FtwTemplate>().is_ok());
            prop_assert_eq!(FtwTemplate::Custom{git_url: template_input.to_string()}, template_input.parse::<FtwTemplate>().unwrap());
        }
    }
}
