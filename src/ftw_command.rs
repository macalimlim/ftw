use crate::ftw_template::FtwTemplate;
use crate::process_command::ProcessCommand;
use crate::traits::Processor;
use crate::traits::ToGitUrl;
use crate::type_alias::Commands;
use crate::type_alias::ProjectName;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub enum FtwCommand {
    New {
        project_name: ProjectName,
        template: FtwTemplate,
    },
}

impl Processor for FtwCommand {
    fn process(&self) {
        match self {
            FtwCommand::New {
                project_name,
                template,
            } => {
                let git_url = &template.to_git_url();
                let gitignore_path: String = format!("{}/.gitignore", project_name);
                let commands: Commands = vec![vec![
                    "cargo",
                    "generate",
                    "--name",
                    project_name,
                    "--git",
                    git_url,
                ]];
                (ProcessCommand { commands: commands }).process();
                let mut gitignore_file = OpenOptions::new()
                    .append(true)
                    .open(gitignore_path)
                    .unwrap();
                if let Err(e) = writeln!(gitignore_file, "godot/export_presets.cfg") {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
    }
}
