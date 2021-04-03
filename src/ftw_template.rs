use crate::traits::ToGitUrl;
use crate::type_alias::GitUrl;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod ftw_template_tests {
    use super::*;
    use proptest::prelude::{prop_assert, prop_assert_eq, prop_assume, proptest};

    #[test]
    fn test_from_str() -> Result<(), ()> {
        let custom_template = "/path/to/custom/template";
        assert_eq!(FtwTemplate::Default, "default".parse()?);
        assert_eq!(
            FtwTemplate::Custom {
                git_url: custom_template.to_string(),
            },
            custom_template.parse()?
        );
        Ok(())
    }

    #[test]
    fn test_to_git_url() {
        let custom_template = "/path/to/custom/template";
        assert_eq!(
            FtwTemplate::Default.to_git_url(),
            "https://github.com/godot-rust/godot-rust-template".to_string()
        );
        assert_eq!(
            FtwTemplate::Custom {
                git_url: custom_template.to_string()
            }
            .to_git_url(),
            custom_template.to_string()
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
