use crate::ftw_build_type::FtwBuildType;
use crate::ftw_error::FtwError;
use crate::ftw_node_type::FtwNodeType;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::process_command::ProcessCommand;
use crate::traits::{
    Processor, ToAppExt, ToCliArg, ToExportArg, ToExportName, ToGitUrl, ToLibExt, ToLibPrefix,
};
use crate::type_alias::{ClassName, Commands, ProjectName};
use crate::util;
use cargo_edit::get_crate_name_from_path;
use fs_extra::dir::CopyOptions;
use fs_extra::{move_items, remove_items};
use kstring::KString;
use liquid::{object, Object, ParserBuilder};
use liquid_core::model::{ScalarCow, Value};
use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, write, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use strum::IntoEnumIterator;
use voca_rs::Voca;

pub enum FtwCommand {
    New {
        project_name: ProjectName,
        template: FtwTemplate,
    },
    Class {
        class_name: ClassName,
        node_type: FtwNodeType,
    },
    Singleton {
        class_name: ClassName,
    },
    Run,
    Build {
        target: FtwTarget,
        build_type: FtwBuildType,
    },
    Export {
        target: FtwTarget,
        build_type: FtwBuildType,
    },
}

impl FtwCommand {
    fn generate_project(project_name: &str, template: &FtwTemplate) -> Result<(), FtwError> {
        let git_url = &template.to_git_url();
        let commands: Commands = vec![vec![
            "cargo",
            "generate",
            "--name",
            project_name,
            "--git",
            git_url,
        ]];
        (ProcessCommand { commands }).process()
    }

    fn append_to_gitignore(project_name: &str) -> Result<(), FtwError> {
        let gitignore_path: String = format!("{}/.gitignore", project_name);
        let mut gitignore_file = OpenOptions::new().append(true).open(gitignore_path)?;
        Ok(writeln!(gitignore_file, "godot/export_presets.cfg")?)
    }

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
        let project_files: Vec<&str> = vec![
            "Cargo.toml",
            "Makefile",
            "godot/default_env.tres",
            "godot/export_presets.cfg",
            "godot/native/game.gdnlib",
            "godot/project.godot",
            "rust/src/lib.rs",
            "rust/Cargo.toml",
        ];
        let targets_to_filter = vec![
            FtwTarget::WindowsX86Gnu,
            FtwTarget::WindowsX86Msvc,
            FtwTarget::WindowsX86_64Gnu,
            FtwTarget::WindowsX86_64Msvc,
        ];
        let targets: Vec<String> = FtwTarget::iter()
            .filter(|t| !targets_to_filter.iter().any(|tf| t == tf))
            .map(|t| {
                let gitkeep = format!("{}/.gitkeep", t.to_cli_arg());
                let bin_gitkeep = format!("bin/{}", gitkeep);
                let lib_gitkeep = format!("lib/{}", gitkeep);
                vec![bin_gitkeep, lib_gitkeep]
            })
            .flatten()
            .collect();
        let is_valid_project = project_files.iter().all(|i| Path::new(i).exists());
        let is_valid_targets = targets.iter().all(|t| Path::new(&t).exists());
        if is_valid_project && is_valid_targets {
            println!("Project is valid...");
            Ok(true)
        } else {
            Err(FtwError::InvalidProject)
        }
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

    fn get_tmpl_globals(class_name: &str, node_type: &FtwNodeType) -> Object {
        object!({
            "class_name": class_name,
            "node_type": node_type.to_string(),
        })
    }

    fn create_lib_rs_file(class_name: &str, node_type: &FtwNodeType) -> Result<(), FtwError> {
        let mut tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
        let module_class_name_pairs = FtwCommand::generate_module_class_name_pairs()?;
        let k = KString::from_ref("module_class_name_pairs");
        let v = Value::Scalar(ScalarCow::from(module_class_name_pairs));
        tmpl_globals.insert(k, v);
        FtwCommand::create_file(
            &String::from_utf8_lossy(include_bytes!("lib_tmpl.rs")),
            "rust/src/lib.rs",
            &tmpl_globals,
        )
    }

    fn create_class_rs_file(class_name: &str, node_type: &FtwNodeType) -> Result<(), FtwError> {
        let tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
        FtwCommand::create_file(
            &String::from_utf8_lossy(include_bytes!("class_tmpl.rs")),
            &format!("rust/src/{}.rs", class_name._snake_case()),
            &tmpl_globals,
        )
    }

    fn create_gdns_file(class_name: &str, node_type: &FtwNodeType) -> Result<(), FtwError> {
        let tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
        FtwCommand::create_file(
            &String::from_utf8_lossy(include_bytes!("gdns_tmpl.gdns")),
            &format!("godot/native/{}.gdns", class_name._pascal_case()),
            &tmpl_globals,
        )
    }

    fn create_tscn_file(class_name: &str, node_type: &FtwNodeType) -> Result<(), FtwError> {
        let tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
        FtwCommand::create_file(
            &String::from_utf8_lossy(include_bytes!("tscn_tmpl.tscn")),
            &format!("godot/scenes/{}.tscn", class_name._pascal_case()),
            &tmpl_globals,
        )
    }

    fn build_lib(target: &FtwTarget, build_type: &FtwBuildType) -> Result<(), FtwError> {
        let crate_name = get_crate_name_from_path("./rust/")?;
        let target_cli_arg = target.to_cli_arg();
        let build_type_cli_arg = build_type.to_cli_arg();
        let target_lib_ext = target.to_lib_ext();
        let source_path = format!(
            "./target/{}/{}/{}{}.{}",
            &target_cli_arg,
            build_type.to_string().to_lowercase(),
            target.to_lib_prefix(),
            crate_name,
            &target_lib_ext
        );
        let target_path = format!("./lib/{}", &target_cli_arg);
        let mut cargo_build_cmd = vec!["cargo", "build", "--target", &target_cli_arg];
        cargo_build_cmd.append(&mut match build_type {
            FtwBuildType::Debug => vec![],
            FtwBuildType::Release => vec![&build_type_cli_arg],
        });
        let commands: Commands = vec![cargo_build_cmd];
        (ProcessCommand { commands }).process()?;
        let target_lib_file = format!(
            "{}/{}{}.{}",
            target_path,
            target.to_lib_prefix(),
            crate_name,
            target.to_lib_ext()
        );
        if Path::new(&target_lib_file).exists() {
            let target_lib_files = vec![target_lib_file];
            remove_items(&target_lib_files)?;
        }
        let options = CopyOptions::new();
        let source_paths = vec![source_path];
        move_items(&source_paths, target_path, &options)?;
        Ok(())
    }

    fn export_game(target: &FtwTarget, build_type: &FtwBuildType) -> Result<(), FtwError> {
        let crate_name = get_crate_name_from_path("./rust/")?;
        let target_cli_arg = target.to_cli_arg();
        let target_export_name = target.to_export_name();
        let build_type_export_arg = build_type.to_export_arg();
        let build_type = build_type.to_string().to_lowercase();
        let target_app_ext = target.to_app_ext();
        let export_preset_name =
            format!("{}.{}.{}", target_export_name, target_cli_arg, build_type);
        let export_path = format!(
            "../bin/{}/{}.{}.{}{}",
            &target_cli_arg, &crate_name, build_type, &target_cli_arg, &target_app_ext
        );
        let commands = vec![vec![
            "godot-headless",
            &build_type_export_arg,
            &export_preset_name,
            &export_path,
        ]];
        env::set_current_dir(Path::new("./godot"))?;
        (ProcessCommand { commands }).process()
    }

    fn run_with_godot() -> Result<(), FtwError> {
        let commands: Commands = vec![vec!["godot", "--path", "godot/", "-d"]];
        (ProcessCommand { commands }).process()
    }
}

impl Processor for FtwCommand {
    fn process(&self) -> Result<(), FtwError> {
        match self {
            FtwCommand::New {
                project_name,
                template,
            } => {
                FtwCommand::generate_project(project_name, &template)?;
                FtwCommand::append_to_gitignore(project_name)
            }
            FtwCommand::Class {
                class_name,
                node_type,
            } => {
                FtwCommand::is_valid_project()?;
                FtwCommand::create_class_rs_file(class_name, &node_type)?;
                FtwCommand::create_gdns_file(class_name, &node_type)?;
                FtwCommand::create_tscn_file(class_name, &node_type)?;
                FtwCommand::create_lib_rs_file(class_name, &node_type)
            }
            FtwCommand::Singleton { class_name } => {
                FtwCommand::is_valid_project()?;
                let node_type = FtwNodeType::Node;
                FtwCommand::create_class_rs_file(class_name, &node_type)?;
                FtwCommand::create_gdns_file(class_name, &node_type)?;
                FtwCommand::create_lib_rs_file(class_name, &node_type)?;
                println!("Open Project -> Project Settings -> Autoload and then add the newly created *.gdns file as an autoload");
                // TODO: parse and modify project.godot file to include the newly created *.gdns file as an autoload
                Ok(())
            }
            FtwCommand::Run => {
                FtwCommand::is_valid_project()?;
                let build_type = FtwBuildType::Debug;
                let current_platform = util::get_current_platform();
                let target = current_platform
                    .parse()
                    .unwrap_or(FtwTarget::WindowsX86_64Msvc);
                FtwCommand::build_lib(&target, &build_type)?;
                FtwCommand::run_with_godot()
            }
            FtwCommand::Build { target, build_type } => {
                FtwCommand::is_valid_project()?;
                FtwCommand::build_lib(target, build_type)
            }

            FtwCommand::Export { target, build_type } => {
                FtwCommand::is_valid_project()?;
                FtwCommand::build_lib(target, build_type)?;
                FtwCommand::export_game(target, build_type)
            }
        }
    }
}
