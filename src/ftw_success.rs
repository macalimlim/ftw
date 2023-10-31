use crate::ftw_build_type::FtwBuildType;
use crate::ftw_machine_type::FtwMachineType;
use crate::ftw_node_type::FtwNodeType;
use crate::ftw_tag::FtwTag;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::traits::ToMessage;
use crate::type_alias::{ClassName, Message, ProjectName};
use colored::{ColoredString, Colorize};

#[derive(Debug, Eq, PartialEq)]
pub enum FtwSuccess<'a> {
    New {
        project_name: ProjectName,
        template: &'a FtwTemplate,
        tag: &'a FtwTag,
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
        targets: &'a Vec<FtwTarget>,
        build_type: &'a FtwBuildType,
    },
    Export {
        targets: &'a Vec<FtwTarget>,
        build_type: &'a FtwBuildType,
    },
    Clean,
}

impl FtwSuccess<'_> {
    const THUMBS_UP: &'static str = "\u{f164}";

    fn get_styled_success() -> ColoredString {
        "SUCCESS:".bold().green()
    }
}

impl ToMessage for FtwSuccess<'_> {
    fn to_message(&self) -> Message {
        let description = match self {
            FtwSuccess::New {
                project_name,
                template,
                tag,
            } => match template {
                FtwTemplate::Default { git_url } | FtwTemplate::Custom { git_url } => {
                    let styled_project_name = project_name.blue().bold().italic();
                    let styled_template = template.to_string().blue().bold().italic();
                    let styled_git_url = git_url.underline();
                    format!("A new project has been created {styled_project_name} using the {styled_template} ({styled_git_url} {tag}) template")
                }
            },
            FtwSuccess::Class {
                class_name,
                node_type,
            } => {
                let styled_class_name = class_name.blue().bold().italic();
                let styled_node_type = node_type.to_string().blue().bold().italic();
                format!("A new class has been created {styled_class_name} using the {styled_node_type} node type")
            }
            FtwSuccess::Singleton { class_name } => {
                let styled_class_name = class_name.blue().bold().italic();
                format!("A new singleton class has been created {styled_class_name}")
            }
            FtwSuccess::Run { machine_type } => {
                let styled_machine_type = machine_type.to_string().blue().bold().italic();
                format!("The game was run as a {styled_machine_type} application")
            }
            FtwSuccess::Build {
                targets,
                build_type,
            } => {
                let targets: Vec<String> = targets
                    .iter()
                    .map(|target| format!("lib/{target}"))
                    .collect();
                let styled_targets = targets.join(",").blue().bold().italic();
                let styled_build_type = build_type.to_string().blue().bold().italic();
                format!(
                    "A library was created at {styled_targets} with a {styled_build_type} profile"
                )
            }
            FtwSuccess::Export {
                targets,
                build_type,
            } => {
                let targets: Vec<String> = targets
                    .iter()
                    .map(|target| format!("bin/{target}"))
                    .collect();
                let styled_target = targets.join(",").blue().bold().italic();
                let styled_build_type = build_type.to_string().blue().bold().italic();
                format!("A game was created at {styled_target} with a {styled_build_type} profile")
            }
            FtwSuccess::Clean => "The project is now clean from excess artifacts".to_string(),
        };
        let thumbs_up = FtwSuccess::THUMBS_UP;
        let styled_success = FtwSuccess::get_styled_success();
        format!("{thumbs_up} {styled_success} {description}")
    }
}

#[cfg(test)]
mod ftw_success_tests {
    use super::*;

    #[test]
    fn test_to_message() {
        let new_game = "my-awesome-game".to_string();
        let default_template = FtwTemplate::default();
        let thumbs_up = FtwSuccess::THUMBS_UP;
        let styled_success = FtwSuccess::get_styled_success();
        let styled_new_game = new_game.blue().bold().italic();
        let styled_default_template = default_template.to_string().blue().bold().italic();
        let tag = FtwTag::default();
        if let FtwTemplate::Default { git_url } = FtwTemplate::default() {
            let styled_git_url = git_url.underline();
            let ftw_success_new_default_message = FtwSuccess::New {
                project_name: new_game.clone(),
                template: &default_template,
                tag: &tag,
            }
            .to_message();
            assert_eq!(
                format!("{thumbs_up} {styled_success} A new project has been created {styled_new_game} using the {styled_default_template} ({styled_git_url} {tag}) template"),
                format!("{ftw_success_new_default_message}")
            );
        }
        //
        let class_name = "IronMan".to_string();
        let node_type = FtwNodeType::Area2D;
        let ftw_success_class_message = FtwSuccess::Class {
            class_name: class_name.clone(),
            node_type: &node_type,
        }
        .to_message();
        let styled_class_name = class_name.blue().bold().italic();
        let styled_node_type = node_type.to_string().blue().bold().italic();
        assert_eq!(
            format!("{thumbs_up} {styled_success} A new class has been created {styled_class_name} using the {styled_node_type} node type"),
            format!("{ftw_success_class_message}")
        );
        //
        let ftw_success_singleton_message = FtwSuccess::Singleton {
            class_name: class_name.clone(),
        }
        .to_message();
        assert_eq!(
            format!("{thumbs_up} {styled_success} A new singleton class has been created {styled_class_name}"),
            format!("{ftw_success_singleton_message}")
        );
        //
        let machine_type = FtwMachineType::Desktop;
        let ftw_success_run_message = FtwSuccess::Run {
            machine_type: &machine_type,
        }
        .to_message();
        let styled_machine_type = machine_type.to_string().blue().bold().italic();
        assert_eq!(
            format!("{thumbs_up} {styled_success} The game was run as a {styled_machine_type} application"),
            format!("{ftw_success_run_message}")
        );
        let machine_type = FtwMachineType::Server;
        let ftw_success_run_message = FtwSuccess::Run {
            machine_type: &machine_type,
        }
        .to_message();
        let styled_machine_type = machine_type.to_string().blue().bold().italic();
        assert_eq!(
            format!("{thumbs_up} {styled_success} The game was run as a {styled_machine_type} application"),
            format!("{ftw_success_run_message}")
        );
        //
        let target = FtwTarget::LinuxX86_64;
        let targets = vec![target];
        let debug = FtwBuildType::Debug;
        let ftw_success_build_debug_message = FtwSuccess::Build {
            targets: &targets,
            build_type: &debug,
        }
        .to_message();
        let styled_target = format!("lib/{target}").blue().bold().italic();
        let styled_debug = debug.to_string().blue().bold().italic();
        assert_eq!(
            format!("{thumbs_up} {styled_success} A library was created at {styled_target} with a {styled_debug} profile"),
            format!("{ftw_success_build_debug_message}")
        );
        //
        let release = FtwBuildType::Release;
        let ftw_success_build_release_message = FtwSuccess::Build {
            targets: &targets,
            build_type: &release,
        }
        .to_message();
        let styled_release = release.to_string().blue().bold().italic();
        assert_eq!(
            format!("{thumbs_up} {styled_success} A library was created at {styled_target} with a {styled_release} profile"),
            format!("{ftw_success_build_release_message}")
        );
        //
        let target = FtwTarget::LinuxX86_64;
        let targets = vec![target];
        let ftw_success_export_debug_message = FtwSuccess::Export {
            targets: &targets,
            build_type: &debug,
        }
        .to_message();
        let styled_target = format!("bin/{target}").blue().bold().italic();
        assert_eq!(
            format!("{thumbs_up} {styled_success} A game was created at {styled_target} with a {styled_debug} profile"),
            format!("{ftw_success_export_debug_message}")
        );
        //
        let ftw_success_export_release_message = FtwSuccess::Export {
            targets: &targets,
            build_type: &release,
        }
        .to_message();
        assert_eq!(
            format!("{thumbs_up} {styled_success} A game was created at {styled_target} with a {styled_release} profile"),
            format!("{ftw_success_export_release_message}")
        );
        //
        let ftw_success_clean_message = FtwSuccess::Clean.to_message();
        assert_eq!(
            format!("{thumbs_up} {styled_success} The project is now clean from excess artifacts"),
            format!("{ftw_success_clean_message}")
        );
    }

    #[test]
    fn test_new_custom_template_to_message() {
        let new_game = "my-awesome-game".to_string();
        let custom_template = FtwTemplate::Custom {
            git_url: "/path/to/custom/template".to_string(),
        };
        let thumbs_up = FtwSuccess::THUMBS_UP;
        let styled_success = FtwSuccess::get_styled_success();
        let styled_new_game = new_game.blue().bold().italic();
        let styled_custom_template = custom_template.to_string().blue().bold().italic();
        let tag = FtwTag::default();
        if let FtwTemplate::Custom { ref git_url } = custom_template {
            let ftw_success_new_custom_message = FtwSuccess::New {
                project_name: new_game.clone(),
                template: &custom_template,
                tag: &tag,
            }
            .to_message();
            let styled_git_url = git_url.underline();
            assert_eq!(
                format!("{thumbs_up} {styled_success} A new project has been created {styled_new_game} using the {styled_custom_template} ({styled_git_url} {tag}) template"),
                format!("{ftw_success_new_custom_message}")
            );
        }
    }
}
