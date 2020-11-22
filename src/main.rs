mod ftw_command;
mod ftw_template;
mod process_command;
mod traits;
mod type_alias;

use crate::ftw_command::FtwCommand;
use crate::ftw_template::FtwTemplate;
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
                             (@arg template: !required "set the template to be used in your project")))
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
        _ => panic!("this should not happen!"),
    };
    command.process();
}
