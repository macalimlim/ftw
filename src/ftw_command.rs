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
use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, write, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use voca_rs::Voca;

pub enum FtwCommand {
    New {
        project_name: ProjectName,
        template: FtwTemplate,
    },
    Class {
        class_name: ClassName,
        node_type: NodeType,
    },
    Singleton {
        class_name: ClassName,
    },
}

impl FtwCommand {
    fn create_file(
        template_contents: &str,
        target_file_path: &str,
        template_globals: &Object,
    ) -> Result<(), FtwError> {
        let builder = ParserBuilder::with_stdlib().build()?;
        let template = builder.parse(template_contents)?;
        let output = template.render(template_globals)?;
        write(target_file_path, output.as_bytes())?;
        println!("{} has been created...", target_file_path);
        Ok(())
    }

    fn is_valid_project() -> Result<bool, FtwError> {
        let project_directory = env::current_dir()?;
        let project_files: Vec<&str> = vec![
            "Cargo.toml",
            "godot/native/game.gdnlib",
            "godot/project.godot",
            "rust/src/lib.rs",
            "rust/Cargo.toml",
        ];
        let is_valid_project = project_files
            .iter()
            .all(|i| Path::new(&format!("{}/{}", project_directory.display(), i)).exists());
        if is_valid_project {
            println!("Project is valid...");
            Ok(true)
        } else {
            Err(FtwError::InvalidProject)
        }
    }

    fn create_files(
        files_to_be_generated: Vec<(Cow<'_, str>, String, &Object)>,
    ) -> Result<(), FtwError> {
        for (template_contents, target_file_path, template_globals) in files_to_be_generated {
            FtwCommand::create_file(&template_contents, &target_file_path, &template_globals)?
        }
        Ok(())
    }

    fn generate_module_class_name_pairs() -> Result<String, FtwError> {
        let godot_native_files = read_dir("godot/native/")?;
        let mut module_class_name_pairs = vec![];
        for file in godot_native_files {
            let file = file?;
            if Path::new(&file.path())
                .extension()
                .and_then(OsStr::to_str)
                .ok_or(FtwError::Utf8ConversionError)?
                == "gdns"
            {
                let class_name = file
                    .file_name()
                    .to_str()
                    .ok_or(FtwError::Utf8ConversionError)?
                    .replace(".gdns", "");
                let module = class_name._snake_case();
                let pair = format!("{},{}", module, class_name);
                module_class_name_pairs.push(pair);
            }
        }
        Ok(module_class_name_pairs.join("|"))
    }

    fn create_lib_rs_file(class_name: &str, node_type: NodeType) -> Result<(), FtwError> {
        let module_class_name_pairs = FtwCommand::generate_module_class_name_pairs()?;
        let tmpl_globals = object!({
            "class_name": class_name,
            "node_type": node_type.to_string(),
            "module_class_name_pairs": module_class_name_pairs,
        });
        FtwCommand::create_file(
            &String::from_utf8_lossy(include_bytes!("lib_tmpl.rs")),
            &"rust/src/lib.rs".to_string(),
            &tmpl_globals,
        )
    }
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
                (ProcessCommand { commands }).process()?;
                let mut gitignore_file = OpenOptions::new().append(true).open(gitignore_path)?;
                Ok(writeln!(gitignore_file, "godot/export_presets.cfg")?)
            }
            FtwCommand::Class {
                class_name,
                node_type,
            } => {
                FtwCommand::is_valid_project()?;
                let tmpl_globals = object!({
                    "class_name": class_name,
                    "node_type": node_type.to_string(),
                });
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
                FtwCommand::create_files(files_to_be_generated)?;
                FtwCommand::create_lib_rs_file(class_name, *node_type)
            }
            FtwCommand::Singleton { class_name } => {
                FtwCommand::is_valid_project()?;
                let node_type = NodeType::Node;
                let tmpl_globals = object!({
                    "class_name": class_name,
                    "node_type": node_type.to_string(),
                });
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
                ];
                FtwCommand::create_files(files_to_be_generated)?;
                FtwCommand::create_lib_rs_file(class_name, node_type)?;
                println!("Open Project -> Project Settings -> Autoload and then add the newly created *.gdns file as an autoload");
                // TODO: parse and modify project.godot file to include the newly created *.gdns file as an autoload
                Ok(())
            }
        }
    }
}
