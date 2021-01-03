use crate::ftw_error::FtwError;
use crate::ftw_template::FtwTemplate;
use crate::node_type::NodeType;
use crate::process_command::ProcessCommand;
use crate::traits::Processor;
use crate::traits::ToGitUrl;
use crate::type_alias::ClassName;
use crate::type_alias::Commands;
use crate::type_alias::ProjectName;
use liquid::{object, Object, ParserBuilder};
use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, write, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use voca_rs::Voca;

fn generate_file(
    template_contents: &String,
    target_file_path: &String,
    template_globals: &Object,
) -> Result<(), FtwError> {
    let builder = ParserBuilder::with_stdlib().build()?;
    let template = builder.parse(template_contents)?;
    let output = template.render(template_globals)?;
    write(target_file_path, output.as_bytes())?;
    println!("{} has been generated...", target_file_path);
    Ok(())
}

pub enum FtwCommand {
    New {
        project_name: ProjectName,
        template: FtwTemplate,
    },
    Class {
        class_name: ClassName,
        node_type: NodeType,
    },
}

impl Processor for FtwCommand {
    fn process(&self) -> Result<(), FtwError> {
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
                (ProcessCommand { commands: commands }).process()?;
                let mut gitignore_file = OpenOptions::new().append(true).open(gitignore_path)?;
                Ok(writeln!(gitignore_file, "godot/export_presets.cfg")?)
            }
            FtwCommand::Class {
                class_name,
                node_type,
            } => {
                let project_directory = env::current_dir()?;
                let project_files: Vec<&str> = vec![
                    "Cargo.toml",
                    "godot/native/game.gdnlib",
                    "godot/project.godot",
                    "rust/src/lib.rs",
                    "rust/Cargo.toml",
                ];
                let is_valid_project = project_files.iter().fold(true, |s, i| {
                    s && Path::new(&format!("{}/{}", project_directory.display(), i)).exists()
                });
                if !is_valid_project {
                    return Err(FtwError::InvalidProject);
                }
                let tmpl_globals = object!({
                    "class_name": class_name,
                    "node_type": node_type.to_string(),
                });
                //
                let files_to_be_generated = vec![
                    (
                        String::from_utf8_lossy(include_bytes!("class_tmpl.rs")),
                        format!("rust/src/{}.rs", class_name.as_str()._snake_case()),
                        &tmpl_globals,
                    ),
                    (
                        String::from_utf8_lossy(include_bytes!("gdns_tmpl.gdns")),
                        format!("godot/native/{}.gdns", class_name),
                        &tmpl_globals,
                    ),
                    (
                        String::from_utf8_lossy(include_bytes!("tscn_tmpl.tscn")),
                        format!("godot/scenes/{}.tscn", class_name),
                        &tmpl_globals,
                    ),
                ];
                for (template_contents, target_file_path, template_globals) in files_to_be_generated
                {
                    generate_file(
                        &template_contents.to_string(),
                        &target_file_path,
                        template_globals,
                    )?
                }
                let godot_native_files = read_dir("godot/native/")?;
                let mut module_class_name_pairs = vec![];
                for file in godot_native_files {
                    let file = file?;
                    if Path::new(&file.path())
                        .extension()
                        .and_then(OsStr::to_str)
                        .ok_or_else(|| FtwError::Utf8ConversionError)?
                        == "gdns"
                    {
                        let class_name = file
                            .file_name()
                            .to_str()
                            .ok_or_else(|| FtwError::Utf8ConversionError)?
                            .replace(".gdns", "");
                        let module = class_name._snake_case();
                        let pair = format!("{},{}", module, class_name);
                        module_class_name_pairs.push(pair);
                    }
                }
                let module_class_name_pairs = module_class_name_pairs.join("|");
                let tmpl_globals = object!({
                    "class_name": class_name,
                    "node_type": node_type.to_string(),
                    "module_class_name_pairs": module_class_name_pairs,
                });
                generate_file(
                    &String::from_utf8_lossy(include_bytes!("lib_tmpl.rs")).to_string(),
                    &"rust/src/lib.rs".to_string(),
                    &tmpl_globals,
                )
            }
        }
    }
}
