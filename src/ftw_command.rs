use crate::ftw_template::FtwTemplate;
use crate::node_type::NodeType;
use crate::process_command::ProcessCommand;
use crate::traits::Processor;
use crate::traits::ToGitUrl;
use crate::type_alias::ClassName;
use crate::type_alias::Commands;
use crate::type_alias::ProjectName;
use itertools::Itertools;
use liquid::{object, Object, ParserBuilder};
use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, write, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use voca_rs::Voca;

fn generate_file(template_contents: &String, target_file_path: &String, template_globals: &Object) {
    let template = ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template_contents)
        .unwrap();
    let output = template.render(template_globals).unwrap();
    write(target_file_path, output.as_bytes()).unwrap();
    println!("{} has been generated...", target_file_path);
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
            FtwCommand::Class {
                class_name,
                node_type,
            } => {
                let project_directory = env::current_dir().unwrap();
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
                    panic!("invalid project!")
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
                let _result = files_to_be_generated
                    .iter()
                    .map(|(template_contents, target_file_path, template_globals)| {
                        generate_file(
                            &template_contents.to_string(),
                            target_file_path,
                            template_globals,
                        )
                    })
                    .collect::<()>();
                let godot_native_paths = read_dir("godot/native/").unwrap();
                let module_class_name_pairs = godot_native_paths
                    .filter(|path| {
                        Path::new(&path.as_ref().unwrap().path())
                            .extension()
                            .and_then(OsStr::to_str)
                            .unwrap()
                            == "gdns"
                    })
                    .map(|path| {
                        let class_name = path
                            .unwrap()
                            .file_name()
                            .to_str()
                            .unwrap()
                            .replace(".gdns", "");
                        let module = class_name._snake_case();
                        format!("{},{}", module, class_name)
                    })
                    .join("|");
                let tmpl_globals = object!({
                    "class_name": class_name,
                    "node_type": node_type.to_string(),
                    "module_class_name_pairs": module_class_name_pairs,
                });
                generate_file(
                    &String::from_utf8_lossy(include_bytes!("lib_tmpl.rs")).to_string(),
                    &"rust/src/lib.rs".to_string(),
                    &tmpl_globals,
                );
            }
        }
    }
}
