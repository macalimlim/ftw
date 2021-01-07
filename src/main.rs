mod ftw_build_type;
mod ftw_command;
mod ftw_error;
mod ftw_target;
mod ftw_template;
mod node_type;
mod process_command;
mod traits;
mod type_alias;
mod util;

use crate::ftw_build_type::FtwBuildType;
use crate::ftw_command::FtwCommand;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::node_type::NodeType;
use crate::traits::Processor;
use clap::{clap_app, crate_authors, crate_version};
use std::env;

fn main() {
    let version = crate_version!();
    let author = crate_authors!("\n");
    let matches = clap_app!(ftw =>
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
                             (about: "run a debug version of the game"))
                            (@subcommand build =>
                             (about: "build the library for a particular platform")
                             (@arg target: !required "target platform to build")
                             (@arg build_type: !required "either a debug or release"))
                            (@subcommand export =>
                             (about: "export the game for a particular platform")
                             (@arg target: !required "target platform to build")
                             (@arg build_type: !required "either a debug or release")))
    .get_matches();
    let command: FtwCommand = match matches.subcommand() {
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
            let node_type: NodeType = args
                .value_of("node_type")
                .unwrap_or("node")
                .parse()
                .unwrap_or(NodeType::Node);
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
        Some(("run", _)) => FtwCommand::Run,
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
    };
    if let Err(e) = command.process() {
        eprintln!("{}", e);
    }
}
