mod ftw_build_type;
mod ftw_command;
mod ftw_compiler;
mod ftw_configuration;
mod ftw_error;
mod ftw_machine_type;
mod ftw_node_type;
mod ftw_success;
mod ftw_tag;
mod ftw_target;
mod ftw_template;
mod run_command;
mod test_util;
mod traits;
mod type_alias;
mod util;

use crate::ftw_command::FtwCommand;
use crate::traits::{Processor, ToMessage};
use clap::{arg, command, crate_name, ArgMatches, Command};
use itertools::Itertools;
use std::env;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), ()> {
    let matches = get_clap_command().get_matches();
    let command = parse_matches(&matches);
    command
        .process()
        .map(|ftw_success| println!("{}", ftw_success.to_message()))
        .map_err(|ftw_error| eprintln!("{}", ftw_error.to_message()))
}

fn get_clap_command() -> Command {
    command!(crate_name!())
        .help_template(
            r#"{before-help}{name} {version}
{about-with-newline}{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"#,
        )
        .subcommand(
            Command::new("new")
                .about("create a new godot-rust project directory")
                .arg(arg!(<project_name> "set the name of your project"))
                .arg(arg!([template] "set the template to be used in your project"))
                .arg(arg!([tag] "it can be any tag defined in the template or 'latest'")),
        )
        .subcommand(
            Command::new("class")
                .about("create a new class to be used by a node")
                .arg(arg!(<class_name> "the name of this class"))
                .arg(arg!([node_type] "the type of the node that this class inherits from")),
        )
        .subcommand(
            Command::new("singleton")
                .about("create a singleton (autoloaded) class")
                .arg(arg!(<class_name> "the name of this class")),
        )
        .subcommand(
            Command::new("run")
                .about("run a debug version of the game")
                .arg(arg!([machine_type] "either desktop or server")),
        )
        .subcommand(
            Command::new("build")
                .about("build the library for a particular platform")
                .arg(arg!([targets] "target platforms to build, separated by ','"))
                .arg(arg!([build_type] "either a debug or release")),
        )
        .subcommand(
            Command::new("export")
                .about("export the game for a particular platform")
                .arg(arg!([targets] "target platform to export"))
                .arg(arg!([build_type] "either a debug or release")),
        )
        .subcommand(Command::new("clean").about("cleans the project from excess artifacts"))
}

fn parse_matches(matches: &ArgMatches) -> FtwCommand {
    match matches.subcommand() {
        Some(("new", args)) => {
            let project_name = args
                .get_one("project_name")
                .unwrap_or(&String::from("my-awesome-game"))
                .to_string();
            let template = args
                .get_one("template")
                .unwrap_or(&String::from("default"))
                .parse()
                .unwrap_or_default();
            let tag = args
                .get_one("tag")
                .unwrap_or(&String::from("latest"))
                .parse()
                .unwrap_or_default();
            FtwCommand::New {
                project_name,
                template,
                tag,
            }
        }
        Some(("class", args)) => {
            let class_name = args
                .get_one("class_name")
                .unwrap_or(&String::from("MyClass"))
                .to_string();
            let node_type = args
                .get_one("node_type")
                .unwrap_or(&String::from("Node"))
                .parse()
                .unwrap_or_default();
            FtwCommand::Class {
                class_name,
                node_type,
            }
        }
        Some(("singleton", args)) => {
            let class_name = args
                .get_one("class_name")
                .unwrap_or(&String::from("MySingleton"))
                .to_string();
            FtwCommand::Singleton { class_name }
        }
        Some(("run", args)) => {
            let machine_type = args
                .get_one("machine_type")
                .unwrap_or(&String::from("desktop"))
                .parse()
                .unwrap_or_default();
            FtwCommand::Run { machine_type }
        }
        Some(("build", args)) => {
            let current_platform = util::get_current_platform();
            let targets = args
                .get_one("targets")
                .unwrap_or(&current_platform)
                .split(',')
                .map(|t| t.parse().unwrap_or_default())
                .sorted()
                .dedup()
                .collect();
            let build_type = args
                .get_one("build_type")
                .unwrap_or(&String::from("debug"))
                .parse()
                .unwrap_or_default();
            FtwCommand::Build {
                targets,
                build_type,
            }
        }
        Some(("export", args)) => {
            let current_platform = util::get_current_platform();
            let targets = args
                .get_one("targets")
                .unwrap_or(&current_platform)
                .split(',')
                .map(|t| t.parse().unwrap_or_default())
                .sorted()
                .dedup()
                .collect();
            let build_type = args
                .get_one("build_type")
                .unwrap_or(&String::from("debug"))
                .parse()
                .unwrap_or_default();
            FtwCommand::Export {
                targets,
                build_type,
            }
        }
        Some(("clean", _args)) => FtwCommand::Clean,
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
    use crate::ftw_tag::FtwTag;
    use crate::ftw_target::FtwTarget;
    use crate::ftw_template::FtwTemplate;
    use crate::util;

    #[test]
    fn test_parse_matches_new() {
        let app = get_clap_command();
        let project_name = "my-awesome-game";
        let args = [crate_name!(), "new", project_name, "default"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::default(),
            tag: FtwTag::default(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_new_no_template() {
        let app = get_clap_command();
        let project_name = "my-awesome-game";
        let args = [crate_name!(), "new", project_name];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::default(),
            tag: FtwTag::default(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_new_custom_template() {
        let app = get_clap_command();
        let project_name = "my-awesome-game";
        let git_url = "/path/to/custom/template";
        let args = [crate_name!(), "new", project_name, git_url];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::New {
            project_name: project_name.to_string(),
            template: FtwTemplate::Custom {
                git_url: git_url.to_string(),
            },
            tag: FtwTag::default(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_class() {
        let app = get_clap_command();
        let class_name = "IronMan";
        let args = [crate_name!(), "class", class_name, "Area2D"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Class {
            class_name: class_name.to_string(),
            node_type: FtwNodeType::Area2D,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_class_no_node_type() {
        let app = get_clap_command();
        let class_name = "IronMan";
        let args = [crate_name!(), "class", class_name];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Class {
            class_name: class_name.to_string(),
            node_type: FtwNodeType::Node,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_singleton() {
        let app = get_clap_command();
        let class_name = "Network";
        let args = [crate_name!(), "singleton", class_name];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Singleton {
            class_name: class_name.to_string(),
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_desktop() {
        let app = get_clap_command();
        let args = [crate_name!(), "run", "desktop"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Desktop,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_server() {
        let app = get_clap_command();
        let args = [crate_name!(), "run", "server"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Server,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_run_no_machine_type() {
        let app = get_clap_command();
        let args = [crate_name!(), "run"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Run {
            machine_type: FtwMachineType::Desktop,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build() {
        let app = get_clap_command();
        let args = [crate_name!(), "build", "linux-x86_64", "debug"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            targets: vec![FtwTarget::LinuxX86_64],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_build() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "build",
            "linux-x86_64,windows-x86_64,macos-x86_64,android-x86_64",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::LinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_build_with_blank() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "build",
            ",windows-x86_64,macos-x86_64,android-x86_64",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_build_with_blankv2() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "build",
            "linux-x86_64,windows-x86_64,macos-x86_64,,android-x86_64",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::LinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_build_with_blankv3() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "build",
            "linux-x86_64,windows-x86_64,macos-x86_64,android-x86_64,",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::LinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build_no_build_type() {
        let app = get_clap_command();
        let args = [crate_name!(), "build", "linux-x86_64"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            targets: vec![FtwTarget::LinuxX86_64],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_build_no_target_and_no_build_type() {
        let app = get_clap_command();
        let args = [crate_name!(), "build"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Build {
            targets: vec![util::get_current_platform().parse().unwrap()],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export() {
        let app = get_clap_command();
        let args = [crate_name!(), "export", "linux-x86_64", "debug"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            targets: vec![FtwTarget::LinuxX86_64],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_export() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "export",
            "linux-x86_64,windows-x86_64,macos-x86_64,android-x86_64",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::LinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_export_with_blank() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "export",
            ",windows-x86_64,macos-x86_64,android-x86_64",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_export_with_blankv2() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "export",
            "linux-x86_64,windows-x86_64,macos-x86_64,,android-x86_64",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::LinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_multi_export_with_blankv3() {
        let app = get_clap_command();
        let args = [
            crate_name!(),
            "export",
            "linux-x86_64,windows-x86_64,macos-x86_64,android-x86_64,",
            "debug",
        ];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            targets: vec![
                FtwTarget::AndroidLinuxX86_64,
                FtwTarget::LinuxX86_64,
                FtwTarget::MacOsX86_64,
                FtwTarget::WindowsX86_64Msvc,
            ],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export_no_build_type() {
        let app = get_clap_command();
        let args = [crate_name!(), "export", "linux-x86_64"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            targets: vec![FtwTarget::LinuxX86_64],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_parse_matches_export_no_target_and_no_build_type() {
        let app = get_clap_command();
        let args = [crate_name!(), "export"];
        let target = util::get_current_platform().parse().unwrap();
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Export {
            targets: vec![target],
            build_type: FtwBuildType::Debug,
        };
        assert_eq!(command, cmd);
    }

    #[test]
    fn test_clean() {
        let app = get_clap_command();
        let args = [crate_name!(), "clean"];
        let matches = app.get_matches_from(args);
        let command = parse_matches(&matches);
        let cmd = FtwCommand::Clean {};
        assert_eq!(command, cmd);
    }
}
