mod ftw_command;
mod ftw_template;
mod node_type;
mod process_command;
mod traits;
mod type_alias;

use crate::ftw_command::FtwCommand;
use crate::ftw_template::FtwTemplate;
use crate::node_type::NodeType;
use crate::traits::Processor;
use clap::{clap_app, crate_authors, crate_version};

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
                             (@arg class_name: +required "set the name of your project")
                             (@arg node_type: !required "the type of the node that this class inherits from")))
    .get_matches();
    let command: FtwCommand = match matches.subcommand() {
        Some(("new", args)) => {
            let template: FtwTemplate = match args
                .value_of("template")
                .and_then(|template| Some(template))
            {
                Some(template) => template.parse().unwrap(),
                None => FtwTemplate::Default,
            };
            FtwCommand::New {
                project_name: args.value_of("project_name").unwrap().to_string(),
                template: template,
            }
        }
        Some(("class", args)) => {
            let node_type: NodeType = match args
                .value_of("node_type")
                .and_then(|node_type| Some(node_type))
            {
                Some(node_type) => match node_type.parse() {
                    Ok(nt) => nt,
                    Err(_) => NodeType::Node,
                },
                None => NodeType::Node,
            };
            FtwCommand::Class {
                class_name: args.value_of("class_name").unwrap().to_string(),
                node_type: node_type,
            }
        }
        _ => panic!("this should not happen!"),
    };
    command.process();
}
