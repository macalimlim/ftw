mod ftw_build_type;
mod ftw_command;
mod ftw_configuration;
mod ftw_error;
mod ftw_machine_type;
mod ftw_node_type;
mod ftw_success;
mod ftw_target;
mod ftw_template;
mod process_command;
mod test_util;
mod traits;
mod type_alias;
mod util;

use crate::ftw_build_type::FtwBuildType;
use crate::ftw_command::FtwCommand;
use crate::ftw_machine_type::FtwMachineType;
use crate::ftw_node_type::FtwNodeType;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::traits::Processor;
use clap::{clap_app, crate_authors, crate_name, crate_version, App, ArgMatches};
use std::env;

fn main() -> Result<(), ()> {
    let matches: ArgMatches = get_clap_app().get_matches();
    let command: FtwCommand = parse_matches(&matches);
    command
        .process()
        .map(|ftw_success| println!("SUCCESS: {}", ftw_success))
        .map_err(|ftw_error| eprintln!("ERROR: {}", ftw_error))
}

fn get_clap_app() -> App<'static> {
    let version = crate_version!();
    let author = crate_authors!("\n");
    clap_app!((crate_name!()) =>
              (version: version)
              (author: author)
              (about: "manage your godot-rust project")
              (@subcommand new =>
                (about: "create a new godot-rust project directory")
                (@arg project_name: +required "set the name of your project")
                (@arg template: !required "set the template to be used in your project"))
              (@subcommand class =>
                (about: "create a new class to be used by a node")
                (@arg class_name: +required "the name of this class")
                (@arg node_type: !required "the type of the node that this class inherits from"))
              (@subcommand singleton =>
                (about: "create a singleton (autoloaded) class")
                (@arg class_name: +required "the name of this class"))
              (@subcommand run =>
                (about: "run a debug version of the game")
                (@arg machine_type: !required "either desktop or server"))
              (@subcommand build =>
                (about: "build the library for a particular platform")
                (@arg target: !required "target platform to build")
                (@arg build_type: !required "either a debug or release"))
              (@subcommand export =>
                (about: "export the game for a particular platform")
                (@arg target: !required "target platform to build")
                (@arg build_type: !required "either a debug or release")))
}

fn parse_matches(matches: &ArgMatches) -> FtwCommand {
    match matches.subcommand() {
        Some(("new", args)) => {
            let project_name = args
                .value_of("project_name")
                .unwrap_or("my-awesome-game")
                .to_string();
            let template: FtwTemplate = args
                .value_of("template")
                .unwrap_or("default")
                .parse()
                .unwrap_or(FtwTemplate::Default);
            FtwCommand::New {
                project_name,
                template,
            }
        }
        Some(("class", args)) => {
            let class_name = args.value_of("class_name").unwrap_or("MyClass").to_string();
            let node_type: FtwNodeType = args
                .value_of("node_type")
                .unwrap_or("Node")
                .parse()
                .unwrap_or(FtwNodeType::Node);
            FtwCommand::Class {
                class_name,
                node_type,
            }
        }
        Some(("singleton", args)) => {
            let class_name = args
                .value_of("class_name")
                .unwrap_or("MySingletonClass")
                .to_string();
            FtwCommand::Singleton { class_name }
        }
        Some(("run", args)) => {
            let machine_type = args
                .value_of("machine_type")
                .unwrap_or("desktop")
                .parse()
                .unwrap_or(FtwMachineType::Desktop);
            FtwCommand::Run { machine_type }
        }
        Some(("build", args)) => {
            let current_platform = util::get_current_platform();
            let target = args
                .value_of("target")
                .unwrap_or(&current_platform)
                .parse()
                .unwrap_or(FtwTarget::WindowsX86_64Msvc);
            let build_type = args
                .value_of("build_type")
                .unwrap_or("debug")
                .parse()
                .unwrap_or(FtwBuildType::Debug);
            FtwCommand::Build { target, build_type }
        }
        Some(("export", args)) => {
            let current_platform = util::get_current_platform();
            let target = args
                .value_of("target")
                .unwrap_or(&current_platform)
                .parse()
                .unwrap_or(FtwTarget::WindowsX86_64Msvc);
            let build_type = args
                .value_of("build_type")
                .unwrap_or("debug")
                .parse()
                .unwrap_or(FtwBuildType::Debug);
            FtwCommand::Export { target, build_type }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use crate::ftw_build_type::FtwBuildType;
    use crate::ftw_command::FtwCommand;
    use crate::ftw_machine_type::FtwMachineType;
    use crate::ftw_node_type::FtwNodeType;
    use crate::ftw_target::FtwTarget;
    use crate::ftw_template::FtwTemplate;
    use crate::util;

    #[test]
    fn test_parse_matches_new() {
        let app = get_clap_app();
        let project_name = "my-awesome-game";
        let arg_vec = vec![crate_name!(), "new", project_name, "default"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::Default,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_new_no_template() {
        let app = get_clap_app();
        let project_name = "my-awesome-game";
        let arg_vec = vec![crate_name!(), "new", project_name];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::Default,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_new_custom_template() {
        let app = get_clap_app();
        let project_name = "my-awesome-game";
        let git_url = "/path/to/custom/template";
        let arg_vec = vec![crate_name!(), "new", project_name, git_url];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::Custom {
                git_url: git_url.to_string(),
            },
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_class() {
        let app = get_clap_app();
        let class_name = "IronMan";
        let arg_vec = vec![crate_name!(), "class", class_name, "Area2D"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Class {
            class_name: class_name.to_string(),
            node_type: FtwNodeType::Area2D,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_class_no_node_type() {
        let app = get_clap_app();
        let class_name = "IronMan";
        let arg_vec = vec![crate_name!(), "class", class_name];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Class {
            class_name: class_name.to_string(),
            node_type: FtwNodeType::Node,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_singleton() {
        let app = get_clap_app();
        let class_name = "Network";
        let arg_vec = vec![crate_name!(), "singleton", class_name];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Singleton {
            class_name: class_name.to_string(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_desktop() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "run", "desktop"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Desktop,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_server() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "run", "server"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Server,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_no_machine_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "run"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Desktop,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "build", "linux-x86_64", "debug"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "build", "linux-x86_64"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build_no_target_and_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "build"];
        let target = util::get_current_platform().parse().unwrap();
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            target,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "export", "linux-x86_64", "debug"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "export", "linux-x86_64"];
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            target: FtwTarget::LinuxX86_64,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export_no_target_and_no_build_type() {
        let app = get_clap_app();
        let arg_vec = vec![crate_name!(), "export"];
        let target = util::get_current_platform().parse().unwrap();
        let matches = app.get_matches_from(arg_vec);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            target,
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }
}
