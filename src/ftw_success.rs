use crate::ftw_build_type::FtwBuildType;
use crate::ftw_machine_type::FtwMachineType;
use crate::ftw_node_type::FtwNodeType;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::traits::{ToGitUrl, ToMessage};
use crate::type_alias::{ClassName, Message, ProjectName};
use colored::{ColoredString, Colorize};

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
    Clean,
}

impl FtwSuccess<'_> {
    const THUMBS_UP: &'static str = "\u{f164}";

    fn get_styled_success() -> ColoredString {
        "SUCCESS:".bold().green()
    }
}

#[rustfmt::skip::macros(format)]
impl ToMessage for FtwSuccess<'_> {
    fn to_message(&self) -> Message {
        let description = match self {
            FtwSuccess::New { project_name, template } => format!("A new project has been created {} using the {} ({}) template", project_name.blue().bold().italic(), template.to_string().blue().bold().italic(), template.to_git_url().underline()),
            FtwSuccess::Class { class_name, node_type } => format!("A new class has been created {} using the {} node type", class_name.blue().bold().italic(), node_type.to_string().blue().bold().italic()),
            FtwSuccess::Singleton { class_name } => format!("A new singleton class has been created {}", class_name.blue().bold().italic()),
            FtwSuccess::Run { machine_type } => format!("The game was run as a {} application", machine_type.to_string().blue().bold().italic()),
            FtwSuccess::Build { target, build_type } => format!("A library was created at lib/{} with a {} profile", target.to_string().blue().bold().italic(), build_type.to_string().blue().bold().italic()),
            FtwSuccess::Export { target, build_type } => format!("A game was created at bin/{} with a {} profile", target.to_string().blue().bold().italic(), build_type.to_string().blue().bold().italic()),
            FtwSuccess::Clean => "The project is now clean from excess artifacts".to_string(),
        };
        format!("{} {} {}", FtwSuccess::THUMBS_UP, FtwSuccess::get_styled_success(), description)
    }
}

#[cfg(test)]
mod ftw_success_tests {
    use super::*;

    #[test]
    fn test_to_message() {
        let new_game = "my-awesome-game".to_string();
        let default_template = FtwTemplate::default();
        let ftw_success_new_default = FtwSuccess::New {
            project_name: new_game.clone(),
            template: &default_template,
        };
        assert_eq!(
            format!(
                "{} {} A new project has been created {} using the {} ({}) template",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                new_game.blue().bold().italic(),
                default_template.to_string().blue().bold().italic(),
                default_template.to_git_url().underline()
            ),
            format!("{}", ftw_success_new_default.to_message())
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
                "{} {} A new project has been created {} using the {} ({}) template",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                new_game.blue().bold().italic(),
                custom_template.to_string().blue().bold().italic(),
                custom_template.to_git_url().underline()
            ),
            format!("{}", ftw_success_new_custom.to_message())
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
                "{} {} A new class has been created {} using the {} node type",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                class_name.blue().bold().italic(),
                node_type.to_string().blue().bold().italic()
            ),
            format!("{}", ftw_success_class.to_message())
        );
        //
        let ftw_success_singleton = FtwSuccess::Singleton {
            class_name: class_name.clone(),
        };
        assert_eq!(
            format!(
                "{} {} A new singleton class has been created {}",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                class_name.blue().bold().italic()
            ),
            format!("{}", ftw_success_singleton.to_message())
        );
        //
        let machine_type = FtwMachineType::Desktop;
        let ftw_success_run = FtwSuccess::Run {
            machine_type: &machine_type,
        };
        assert_eq!(
            format!(
                "{} {} The game was run as a {} application",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                machine_type.to_string().blue().bold().italic()
            ),
            format!("{}", ftw_success_run.to_message())
        );
        let machine_type = FtwMachineType::Server;
        let ftw_success_run = FtwSuccess::Run {
            machine_type: &machine_type,
        };
        assert_eq!(
            format!(
                "{} {} The game was run as a {} application",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                machine_type.to_string().blue().bold().italic()
            ),
            format!("{}", ftw_success_run.to_message())
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
                "{} {} A library was created at lib/{} with a {} profile",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                target.to_string().blue().bold().italic(),
                debug.to_string().blue().bold().italic()
            ),
            format!("{}", ftw_success_build_debug.to_message())
        );
        //
        let release = FtwBuildType::Release;
        let ftw_success_build_release = FtwSuccess::Build {
            target: &target,
            build_type: &release,
        };
        assert_eq!(
            format!(
                "{} {} A library was created at lib/{} with a {} profile",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                target.to_string().blue().bold().italic(),
                release.to_string().blue().bold().italic()
            ),
            format!("{}", ftw_success_build_release.to_message())
        );
        //
        let ftw_success_export_debug = FtwSuccess::Export {
            target: &target,
            build_type: &debug,
        };
        assert_eq!(
            format!(
                "{} {} A game was created at bin/{} with a {} profile",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                target.to_string().blue().bold().italic(),
                debug.to_string().blue().bold().italic()
            ),
            format!("{}", ftw_success_export_debug.to_message())
        );
        //
        let ftw_success_export_release = FtwSuccess::Export {
            target: &target,
            build_type: &release,
        };
        assert_eq!(
            format!(
                "{} {} A game was created at bin/{} with a {} profile",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success(),
                target.to_string().blue().bold().italic(),
                release.to_string().blue().bold().italic()
            ),
            format!("{}", ftw_success_export_release.to_message())
        );
        //
        let ftw_success_clean = FtwSuccess::Clean;
        assert_eq!(
            format!(
                "{} {} The project is now clean from excess artifacts",
                FtwSuccess::THUMBS_UP,
                FtwSuccess::get_styled_success()
            ),
            format!("{}", ftw_success_clean.to_message())
        );
    }
}
