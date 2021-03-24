use crate::ftw_build_type::FtwBuildType;
use crate::ftw_configuration::FtwConfiguration;
use crate::ftw_error::FtwError;
use crate::ftw_machine_type::FtwMachineType;
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
use cargo_generate::{generate, Args, Vcs};
use fs_extra::dir::CopyOptions;
use fs_extra::{move_items, remove_items};
use kstring::KString;
use liquid::{object, Object, ParserBuilder};
use liquid_core::model::{ScalarCow, Value};
use regex::Regex;
use std::env;
use std::fs::{create_dir_all, read_dir, write, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use strum::IntoEnumIterator;
use voca_rs::Voca;
use walkdir::WalkDir;

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
    Run {
        machine_type: FtwMachineType,
    },
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
        let args = Args {
            git: Some(git_url.to_string()),
            branch: None,
            name: Some(project_name.to_string()),
            force: false,
            verbose: false,
            config: None,
            favorite: None,
            list_favorites: false,
            silent: false,
            template_values_file: None,
            vcs: Vcs::Git,
        };
        Ok(generate(args)?)
    }

    fn append_to_gitignore(project_name: &str) -> Result<(), FtwError> {
        let gitignore_path: String = format!("{}/.gitignore", project_name);
        let mut gitignore_file = OpenOptions::new().append(true).open(gitignore_path)?;
        let things_to_be_ignored = vec![".ftw", "bin/*", "godot/export_presets.cfg", "lib/*"];
        for thing in things_to_be_ignored {
            writeln!(gitignore_file, "{}", thing)?;
        }
        Ok(())
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

    fn is_linux(target: &FtwTarget) -> Result<bool, FtwError> {
        if *target != FtwTarget::LinuxX86_64 {
            Err(FtwError::UnsupportedTarget)
        } else {
            Ok(true)
        }
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
        let targets: Vec<String> = FtwTarget::iter()
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

    pub fn is_derving_native_class(contents: String) -> Result<bool, FtwError> {
        let reg_ex = Regex::new(r"#\[derive\([a-zA-Z, ]*NativeClass[a-zA-Z, ]*\)\]+")?;
        match reg_ex.find(&contents) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn get_classes_from_directory(directory: &str) -> Result<String, FtwError> {
        let mut classes: Vec<String> = Vec::new();
        for entry in WalkDir::new(directory) {
            let entry = entry?;
            let entry_path = entry.path();
            let path = Path::new(&entry_path);
            let is_file = path.is_file();
            if is_file {
                let mut file_contents = String::new();
                let mut file = File::open(&entry.path())?;
                file.read_to_string(&mut file_contents)?;
                let is_native_class = FtwCommand::is_derving_native_class(file_contents)?;
                if is_native_class {
                    let class_name = path.file_stem().ok_or(FtwError::PathError)?;
                    let class_name = class_name.to_str().ok_or(FtwError::StringConversionError)?;
                    let class_name = class_name.replace(".rs", "")._pascal_case();
                    let module_name = class_name._snake_case();
                    let path_display = format!("{}", path.display());
                    let full_module_name_vec: Vec<&str> =
                        path_display.as_str().split('/').collect();
                    let (_, module_path) = full_module_name_vec.split_at(2);
                    let mut full_module_name_vec = module_path.to_vec();
                    full_module_name_vec.pop();
                    full_module_name_vec.push(&module_name);
                    full_module_name_vec.push(&class_name);
                    classes.push(full_module_name_vec.join("::"));
                }
            }
        }
        Ok(classes.join("|"))
    }

    fn get_tmpl_globals(class_name: &str, node_type: &FtwNodeType) -> Object {
        object!({
            "class_name": class_name,
            "node_type": node_type.to_string(),
        })
    }

    fn create_lib_rs_file(class_name: &str, node_type: &FtwNodeType) -> Result<(), FtwError> {
        let mut tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
        let modules = FtwCommand::get_modules_from_directory("rust/src")?;
        let k = KString::from_ref("modules");
        let v = Value::Scalar(ScalarCow::from(modules));
        tmpl_globals.insert(k, v);
        let classes = FtwCommand::get_classes_from_directory("rust/src")?;
        let k = KString::from_ref("classes");
        let v = Value::Scalar(ScalarCow::from(classes));
        tmpl_globals.insert(k, v);
        FtwCommand::create_file(
            &String::from_utf8_lossy(include_bytes!("templates/lib_tmpl.rs")),
            "rust/src/lib.rs",
            &tmpl_globals,
        )
    }

    fn create_directory(base_path: String, directories: &[String]) -> Result<String, FtwError> {
        let dir_path = directories.join("/");
        let full_path = format!("{}/{}", base_path, dir_path);
        create_dir_all(&full_path)?;
        Ok(full_path)
    }

    fn get_modules_from_directory(directory: &str) -> Result<String, FtwError> {
        let files_and_folders = read_dir(directory)?;
        let mut modules: Vec<String> = Vec::new();
        for entry in files_and_folders {
            let entry = entry?;
            let entry_path = entry.path();
            let path = Path::new(&entry_path);
            let is_file = path.is_file();
            if is_file {
                let mut file_contents = String::new();
                let mut file = File::open(&entry.path())?;
                file.read_to_string(&mut file_contents)?;
                let is_native_class = FtwCommand::is_derving_native_class(file_contents)?;
                if is_native_class {
                    let path = path.file_stem().ok_or(FtwError::PathError)?;
                    modules.push(
                        path.to_os_string()
                            .to_str()
                            .ok_or(FtwError::StringConversionError)?
                            .to_string(),
                    );
                }
            }
            let is_dir = path.is_dir();
            let mod_rs_file = format!("{}/mod.rs", entry_path.as_path().display());
            let mod_rs_file_path = Path::new(&mod_rs_file);
            let is_contains_mod_rs = mod_rs_file_path.exists();
            if is_dir && is_contains_mod_rs {
                modules.push(
                    path.file_name()
                        .ok_or(FtwError::PathError)?
                        .to_str()
                        .ok_or(FtwError::StringConversionError)?
                        .to_string(),
                );
            }
        }
        Ok(modules.join("|"))
    }

    fn create_mod_rs_file(base_src_path: &str, directories: &[String]) -> Result<(), FtwError> {
        if directories.is_empty() {
            Ok(())
        } else {
            let dir = directories.join("/");
            let current_path = format!("{}/{}", &base_src_path, &dir);
            let mod_rs_file = format!("{}/mod.rs", &current_path);
            let modules = FtwCommand::get_modules_from_directory(&current_path)?;
            let tmpl_globals = object!({
                "modules": modules,
            });
            FtwCommand::create_file(
                &String::from_utf8_lossy(include_bytes!("templates/mod_tmpl.rs")),
                &mod_rs_file,
                &tmpl_globals,
            )?;
            match directories.split_last() {
                Some((_, init)) => FtwCommand::create_mod_rs_file(&base_src_path, &init.to_vec()),
                None => Ok(()),
            }
        }
    }

    fn create_class_rs_file(
        class_name: &str,
        directories: &[String],
        node_type: &FtwNodeType,
    ) -> Result<(), FtwError> {
        let base_src_path = "rust/src".to_string();
        let src_dir_path = FtwCommand::create_directory(base_src_path.clone(), &directories)?;
        let class_rs_file = format!("{}/{}.rs", &src_dir_path, class_name._snake_case());
        if !Path::new(&class_rs_file).exists() {
            let tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
            FtwCommand::create_file(
                &String::from_utf8_lossy(include_bytes!("templates/class_tmpl.rs")),
                &class_rs_file,
                &tmpl_globals,
            )?;
        }
        FtwCommand::create_mod_rs_file(&base_src_path, &directories)?;
        Ok(())
    }

    fn create_gdns_file(
        class_name: &str,
        directories: &[String],
        node_type: &FtwNodeType,
    ) -> Result<(), FtwError> {
        let gdns_dir_path = FtwCommand::create_directory("godot/native".to_string(), &directories)?;
        let gdns_file = format!("{}/{}.gdns", &gdns_dir_path, class_name._pascal_case());
        if !Path::new(&gdns_file).exists() {
            let tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
            FtwCommand::create_file(
                &String::from_utf8_lossy(include_bytes!("templates/gdns_tmpl.gdns")),
                &gdns_file,
                &tmpl_globals,
            )?;
        }
        Ok(())
    }

    fn create_tscn_file(
        class_name: &str,
        directories: &[String],
        node_type: &FtwNodeType,
    ) -> Result<(), FtwError> {
        let tscn_dir_path = FtwCommand::create_directory("godot/scenes".to_string(), &directories)?;
        let tscn_file = format!("{}/{}.tscn", &tscn_dir_path, class_name._pascal_case());
        if !Path::new(&tscn_file).exists() {
            let mut tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
            let k = KString::from_ref("dir_path");
            let v = Value::Scalar(ScalarCow::from(if directories.is_empty() {
                "".to_string()
            } else {
                let mut dir = directories.join("/");
                dir.push('/');
                dir
            }));
            tmpl_globals.insert(k, v);
            FtwCommand::create_file(
                &String::from_utf8_lossy(include_bytes!("templates/tscn_tmpl.tscn")),
                &tscn_file,
                &tmpl_globals,
            )?;
        }
        Ok(())
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
        let godot_executable = util::get_godot_exe_for_exporting();
        let commands = vec![vec![
            godot_executable.as_str(),
            &build_type_export_arg,
            &export_preset_name,
            &export_path,
        ]];
        env::set_current_dir(Path::new("./godot"))?;
        (ProcessCommand { commands }).process()
    }

    fn run_with_godot(machine_type: &FtwMachineType) -> Result<(), FtwError> {
        let ftw_cfg = FtwConfiguration::new();
        let (godot_executable, debug_flag) = match machine_type {
            FtwMachineType::Server => (ftw_cfg.godot_server_executable, ""),
            FtwMachineType::Desktop => (ftw_cfg.godot_executable, "-d"),
        };
        let commands: Commands = vec![vec![&godot_executable, "--path", "godot/", debug_flag]];
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
                let (class_name, directories) = util::get_class_name_and_directories(class_name);
                FtwCommand::create_class_rs_file(&class_name, &directories, &node_type)?;
                FtwCommand::create_gdns_file(&class_name, &directories, &node_type)?;
                FtwCommand::create_tscn_file(&class_name, &directories, &node_type)?;
                FtwCommand::create_lib_rs_file(&class_name, &node_type)
            }
            FtwCommand::Singleton { class_name } => {
                FtwCommand::is_valid_project()?;
                let node_type = FtwNodeType::Node;
                let (class_name, directories) = util::get_class_name_and_directories(class_name);
                FtwCommand::create_class_rs_file(&class_name, &directories, &node_type)?;
                FtwCommand::create_gdns_file(&class_name, &directories, &node_type)?;
                FtwCommand::create_lib_rs_file(&class_name, &node_type)?;
                println!("Open Project -> Project Settings -> Autoload and then add the newly created *.gdns file as an autoload");
                // TODO: parse and modify project.godot file to include the newly created *.gdns file as an autoload
                Ok(())
            }
            FtwCommand::Run { machine_type } => {
                FtwCommand::is_valid_project()?;
                let build_type = FtwBuildType::Debug;
                let current_platform = util::get_current_platform();
                let target = current_platform
                    .parse()
                    .unwrap_or(FtwTarget::WindowsX86_64Msvc);
                if *machine_type == FtwMachineType::Server {
                    FtwCommand::is_linux(&target)?;
                }
                FtwCommand::build_lib(&target, &build_type)?;
                FtwCommand::run_with_godot(machine_type)
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
