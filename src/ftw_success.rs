use crate::ftw_build_type::FtwBuildType;
use crate::ftw_machine_type::FtwMachineType;
use crate::ftw_node_type::FtwNodeType;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::type_alias::{ClassName, ProjectName};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum FtwSuccess<'a> {
    New {
        project_name: ProjectName,
        template: &'a FtwTemplate,
    },
    Class {
        class_name: ClassName,
        node_type: &'a FtwNodeType,
    },
    Singleton {
        class_name: ClassName,
    },
    Run {
        machine_type: &'a FtwMachineType,
    },
    Build {
        target: &'a FtwTarget,
        build_type: &'a FtwBuildType,
    },
    Export {
        target: &'a FtwTarget,
        build_type: &'a FtwBuildType,
    },
}

impl Display for FtwSuccess<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message: String = match self {
            FtwSuccess::New {
                project_name,
                template,
            } => format!(
                "A new project has been created {} using {} template",
                project_name, template
            ),
            FtwSuccess::Class {
                class_name,
                node_type,
            } => format!(
                "A new class has been created {} using the {} node type",
                class_name, node_type
            ),
            FtwSuccess::Singleton { class_name } => {
                format!("A new singleton class has been created {}", class_name)
            }
            FtwSuccess::Run { machine_type } => {
                format!("The game was run as a {} application", machine_type)
            }
            FtwSuccess::Build { target, build_type } => {
                format!(
                    "A library was created at lib/{} with a {} profile",
                    target, build_type
                )
            }
            FtwSuccess::Export { target, build_type } => {
                format!(
                    "A game was created at bin/{} with a {} profile",
                    target, build_type
                )
            }
        };
        write!(f, "{}", message)
    }
}

#[cfg(test)]
mod ftw_success_tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let new_game = "my-awesome-game".to_string();
        let default_template = FtwTemplate::Default;
        let ftw_success_new_default = FtwSuccess::New {
            project_name: new_game.clone(),
            template: &default_template,
        };
        assert_eq!(
            format!(
                "A new project has been created {} using {} template",
                new_game, default_template
            ),
            format!("{}", ftw_success_new_default)
        );
        //
        let custom_template = FtwTemplate::Custom {
            git_url: "/path/to/custom/template".to_string(),
        };
        let ftw_success_new_custom = FtwSuccess::New {
            project_name: new_game.clone(),
            template: &custom_template,
        };
        assert_eq!(
            format!(
                "A new project has been created {} using {} template",
                new_game, custom_template
            ),
            format!("{}", ftw_success_new_custom)
        );
        //
        let class_name = "IronMan".to_string();
        let node_type = FtwNodeType::Area2D;
        let ftw_success_class = FtwSuccess::Class {
            class_name: class_name.clone(),
            node_type: &node_type,
        };
        assert_eq!(
            format!(
                "A new class has been created {} using the {} node type",
                class_name, node_type
            ),
            format!("{}", ftw_success_class)
        );
        //
        let ftw_success_singleton = FtwSuccess::Singleton {
            class_name: class_name.clone(),
        };
        assert_eq!(
            format!("A new singleton class has been created {}", class_name),
            format!("{}", ftw_success_singleton)
        );
        //
        let target = FtwTarget::LinuxX86_64;
        let debug = FtwBuildType::Debug;
        let ftw_success_build_debug = FtwSuccess::Build {
            target: &target,
            build_type: &debug,
        };
        assert_eq!(
            format!(
                "A library was created at lib/{} with a {} profile",
                target, debug
            ),
            format!("{}", ftw_success_build_debug)
        );
        //
        let release = FtwBuildType::Release;
        let ftw_success_build_release = FtwSuccess::Build {
            target: &target,
            build_type: &release,
        };
        assert_eq!(
            format!(
                "A library was created at lib/{} with a {} profile",
                target, release
            ),
            format!("{}", ftw_success_build_release)
        );
        //
        let ftw_success_export_debug = FtwSuccess::Export {
            target: &target,
            build_type: &debug,
        };
        assert_eq!(
            format!(
                "A game was created at bin/{} with a {} profile",
                target, debug
            ),
            format!("{}", ftw_success_export_debug)
        );
        //
        let ftw_success_export_release = FtwSuccess::Export {
            target: &target,
            build_type: &release,
        };
        assert_eq!(
            format!(
                "A game was created at bin/{} with a {} profile",
                target, release
            ),
            format!("{}", ftw_success_export_release)
        );
    }
}
