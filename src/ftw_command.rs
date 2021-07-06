use crate::ftw_build_type::FtwBuildType;
use crate::ftw_configuration::FtwConfiguration;
use crate::ftw_error::FtwError;
use crate::ftw_machine_type::FtwMachineType;
use crate::ftw_node_type::FtwNodeType;
use crate::ftw_success::FtwSuccess;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::traits::{
    Processor, Runner, ToAppExt, ToCliArg, ToExportArg, ToExportName, ToGitUrl, ToLibExt,
    ToLibPrefix,
};
use crate::type_alias::{ClassName, ProjectName};
use crate::util;
use cargo_edit::get_crate_name_from_path;
use cargo_generate::{generate, Args, Vcs};
use command_macros::cmd;
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

#[derive(Debug, PartialEq)]
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
            silent: true,
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

    fn delete_items(project_name: &str) -> Result<(), FtwError> {
        let files_to_be_removed: Vec<String> = vec![".travis.yml", "LICENSE", "sh"]
            .into_iter()
            .map(|i| format!("{}/{}", project_name, i))
            .collect();
        Ok(remove_items(&files_to_be_removed)?)
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
        if *target == FtwTarget::LinuxX86_64 {
            Ok(true)
        } else {
            Err(FtwError::UnsupportedTarget)
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
            .flat_map(|t| {
                let gitkeep = format!("{}/.gitkeep", t.to_cli_arg());
                let bin_gitkeep = format!("bin/{}", gitkeep);
                let lib_gitkeep = format!("lib/{}", gitkeep);
                vec![bin_gitkeep, lib_gitkeep]
            })
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

    /// # Errors
    ///
    /// Will return `Err` if the regular expression is invalid
    pub fn is_derving_native_class(contents: &str) -> Result<bool, FtwError> {
        let reg_ex = Regex::new(r"#\[derive\([a-zA-Z, ]*NativeClass[a-zA-Z, ]*\)\]+")?;
        Ok(reg_ex.find(&contents).is_some())
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
                let is_native_class = FtwCommand::is_derving_native_class(&file_contents)?;
                if is_native_class {
                    let class_name = path.file_stem().ok_or(FtwError::PathError)?;
                    let class_name = class_name.to_str().ok_or(FtwError::StringConversionError)?;
                    let class_name = class_name.replace(".rs", "")._pascal_case();
                    let module_name = class_name._snake_case();
                    let path_display = format!("{}", path.display());
                    let replaced_path_display = path_display.as_str().replace("\\", "/");
                    let full_module_name_vec: Vec<&str> =
                        replaced_path_display.split('/').collect();
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

    fn get_tmpl_globals(class_name: &str, node_type: FtwNodeType) -> Object {
        object!({
            "class_name": class_name,
            "node_type": node_type.to_string(),
        })
    }

    fn create_lib_rs_file(class_name: &str, node_type: FtwNodeType) -> Result<(), FtwError> {
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

    fn create_directory(base_path: &str, directories: &[String]) -> Result<String, FtwError> {
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
                let is_native_class = FtwCommand::is_derving_native_class(&file_contents)?;
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
                _ => unreachable!(),
            }
        }
    }

    fn create_class_rs_file(
        class_name: &str,
        directories: &[String],
        node_type: FtwNodeType,
    ) -> Result<(), FtwError> {
        let base_src_path = "rust/src";
        let src_dir_path = FtwCommand::create_directory(base_src_path, &directories)?;
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
        node_type: FtwNodeType,
    ) -> Result<(), FtwError> {
        let gdns_dir_path = FtwCommand::create_directory("godot/native", &directories)?;
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
        node_type: FtwNodeType,
    ) -> Result<(), FtwError> {
        let tscn_dir_path = FtwCommand::create_directory("godot/scenes", &directories)?;
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
        let build_type_cli_arg_option = build_type.to_cli_arg_option();
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
        cmd!(cargo build ("--target") (target_cli_arg) if let Some(btca) = (build_type_cli_arg_option) { (btca) } )
            .run()?;
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
        let current_platform = util::get_current_platform()
            .parse()
            .unwrap_or(FtwTarget::WindowsX86_64Msvc);
        let godot_executable = util::get_godot_exe_for_exporting(&current_platform);
        env::set_current_dir(Path::new("./godot"))?;
        cmd!((godot_executable.as_str())(build_type_export_arg)(
            export_preset_name
        )(export_path))
        .run()
    }

    fn run_with_godot(machine_type: &FtwMachineType) -> Result<(), FtwError> {
        let ftw_cfg = FtwConfiguration::new();
        let (godot_executable, debug_flag) = match machine_type {
            FtwMachineType::Server => (ftw_cfg.godot_server_executable, ""),
            FtwMachineType::Desktop => (ftw_cfg.godot_executable, "-d"),
        };
        cmd!((godot_executable)("--path")("godot/")(debug_flag)).run()
    }
}

impl Processor for FtwCommand {
    fn process(&self) -> Result<FtwSuccess, FtwError> {
        match self {
            FtwCommand::New {
                project_name,
                template,
            } => {
                FtwCommand::generate_project(project_name, &template)?;
                FtwCommand::append_to_gitignore(project_name)?;
                FtwCommand::delete_items(project_name)?;
                Ok(FtwSuccess::New {
                    project_name: project_name.to_string(),
                    template,
                })
            }
            FtwCommand::Class {
                class_name,
                node_type,
            } => {
                FtwCommand::is_valid_project()?;
                let (class_name, directories) = util::get_class_name_and_directories(class_name);
                FtwCommand::create_class_rs_file(&class_name, &directories, *node_type)?;
                FtwCommand::create_gdns_file(&class_name, &directories, *node_type)?;
                FtwCommand::create_tscn_file(&class_name, &directories, *node_type)?;
                FtwCommand::create_lib_rs_file(&class_name, *node_type)?;
                Ok(FtwSuccess::Class {
                    class_name,
                    node_type,
                })
            }
            FtwCommand::Singleton { class_name } => {
                FtwCommand::is_valid_project()?;
                let node_type = FtwNodeType::Node;
                let (class_name, directories) = util::get_class_name_and_directories(class_name);
                FtwCommand::create_class_rs_file(&class_name, &directories, node_type)?;
                FtwCommand::create_gdns_file(&class_name, &directories, node_type)?;
                FtwCommand::create_lib_rs_file(&class_name, node_type)?;
                println!("Open Project -> Project Settings -> Autoload and then add the newly created *.gdns file as an autoload");
                // TODO: parse and modify project.godot file to include the newly created *.gdns file as an autoload
                Ok(FtwSuccess::Singleton { class_name })
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
                FtwCommand::run_with_godot(machine_type)?;
                Ok(FtwSuccess::Run { machine_type })
            }
            FtwCommand::Build { target, build_type } => {
                FtwCommand::is_valid_project()?;
                FtwCommand::build_lib(target, build_type)?;
                Ok(FtwSuccess::Build { target, build_type })
            }

            FtwCommand::Export { target, build_type } => {
                FtwCommand::is_valid_project()?;
                FtwCommand::build_lib(target, build_type)?;
                FtwCommand::export_game(target, build_type)?;
                Ok(FtwSuccess::Export { target, build_type })
            }
        }
    }
}

#[cfg(test)]
mod ftw_command_tests {
    use super::*;

    use crate::test_util::Project;
    use std::env;

    #[test]
    fn test_is_linux() {
        let res = FtwCommand::is_linux(&FtwTarget::LinuxX86_64);
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[test]
    fn test_is_linux_not_linux() {
        let res = FtwCommand::is_linux(&FtwTarget::WindowsX86_64Msvc);
        match res {
            Err(FtwError::UnsupportedTarget) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_is_valid_project_no_cargo_toml() {
        let project = Project::default();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let _ = remove_items(&vec!["Cargo.toml"]);
        let res = FtwCommand::is_valid_project();
        match res {
            Err(FtwError::InvalidProject) => assert!(true),
            _ => unreachable!(),
        }
        let _ = env::set_current_dir(Path::new("../"));
        drop(project)
    }

    #[test]
    fn test_process_ftw_command_new() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        assert!(project.exists(".gitignore"));
        assert!(project.exists("Cargo.toml"));
        assert!(project.exists("Makefile"));
        assert!(project.exists("godot/default_env.tres"));
        assert!(project.exists("godot/export_presets.cfg"));
        assert!(project.exists("godot/native/game.gdnlib"));
        assert!(project.exists("godot/project.godot"));
        assert!(project.exists("rust/Cargo.toml"));
        assert!(project.exists("rust/src/lib.rs"));
        assert!(!project.exists("LICENSE"));
        assert!(!project.exists(".travis.yml"));
        assert!(!project.exists("sh"));
        assert!(project.read(".gitignore").contains(".ftw"));
        assert!(project.read(".gitignore").contains("bin/*"));
        assert!(project.read(".gitignore").contains("export_presets.cfg"));
        assert!(project.read(".gitignore").contains("lib/*"));
        assert!(project
            .read("rust/Cargo.toml")
            .contains(&project.get_name()));
    }

    #[test]
    fn test_process_ftw_command_class() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let cmd = FtwCommand::Class {
            class_name: "MyPlayer".to_string(),
            node_type: FtwNodeType::Area2D,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
        assert!(project.exists("rust/src/my_player.rs"));
        assert!(project.exists("godot/native/MyPlayer.gdns"));
        assert!(project.exists("godot/scenes/MyPlayer.tscn"));
        assert!(project.exists("rust/src/lib.rs"));
        assert!(project
            .read("rust/src/my_player.rs")
            .contains("pub struct MyPlayer"));
        assert!(project
            .read("rust/src/my_player.rs")
            .contains("#[inherit(Area2D)]"));
        assert!(project
            .read("godot/native/MyPlayer.gdns")
            .contains("resource_name = \"MyPlayer\""));
        assert!(project
            .read("godot/native/MyPlayer.gdns")
            .contains("class_name = \"MyPlayer\""));
        assert!(project
            .read("godot/scenes/MyPlayer.tscn")
            .contains("[ext_resource path=\"res://native/MyPlayer.gdns\" type=\"Script\" id=1]"));
        assert!(project
            .read("godot/scenes/MyPlayer.tscn")
            .contains("[node name=\"MyPlayer\" type=\"Area2D\"]"));
        assert!(project.read("rust/src/lib.rs").contains("mod my_player;"));
        assert!(project
            .read("rust/src/lib.rs")
            .contains("handle.add_class::<my_player::MyPlayer>();"));
    }

    #[test]
    fn test_process_ftw_command_class_with_subs() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let cmd = FtwCommand::Class {
            class_name: "foo/bar/baz/MyPlayer".to_string(),
            node_type: FtwNodeType::Area2D,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
        assert!(project.exists("rust/src/foo/bar/baz/my_player.rs"));
        assert!(project.exists("rust/src/foo/bar/baz/mod.rs"));
        assert!(project.exists("rust/src/foo/bar/mod.rs"));
        assert!(project.exists("rust/src/foo/mod.rs"));
        assert!(project.exists("godot/native/foo/bar/baz/MyPlayer.gdns"));
        assert!(project.exists("godot/scenes/foo/bar/baz/MyPlayer.tscn"));
        assert!(project.exists("rust/src/lib.rs"));
        assert!(project
            .read("rust/src/foo/bar/baz/my_player.rs")
            .contains("pub struct MyPlayer"));
        assert!(project
            .read("rust/src/foo/bar/baz/my_player.rs")
            .contains("#[inherit(Area2D)]"));
        assert!(project
            .read("godot/native/foo/bar/baz/MyPlayer.gdns")
            .contains("resource_name = \"MyPlayer\""));
        assert!(project
            .read("godot/native/foo/bar/baz/MyPlayer.gdns")
            .contains("class_name = \"MyPlayer\""));
        assert!(project
            .read("godot/scenes/foo/bar/baz/MyPlayer.tscn")
            .contains(
            "[ext_resource path=\"res://native/foo/bar/baz/MyPlayer.gdns\" type=\"Script\" id=1]"
        ));
        assert!(project
            .read("godot/scenes/foo/bar/baz/MyPlayer.tscn")
            .contains("[node name=\"MyPlayer\" type=\"Area2D\"]"));
        assert!(project.read("rust/src/lib.rs").contains("mod foo;"));
        assert!(project
            .read("rust/src/lib.rs")
            .contains("handle.add_class::<foo::bar::baz::my_player::MyPlayer>();"));
        assert!(project
            .read("rust/src/foo/bar/baz/mod.rs")
            .contains("pub mod my_player;"));
        assert!(project
            .read("rust/src/foo/bar/mod.rs")
            .contains("pub mod baz;"));
        assert!(project.read("rust/src/foo/mod.rs").contains("pub mod bar;"));
    }

    #[test]
    fn test_process_ftw_command_singleton() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let cmd = FtwCommand::Singleton {
            class_name: "MyPlayer".to_string(),
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
        assert!(project.exists("rust/src/my_player.rs"));
        assert!(project.exists("godot/native/MyPlayer.gdns"));
        assert!(project.exists("rust/src/lib.rs"));
        assert!(project
            .read("rust/src/my_player.rs")
            .contains("pub struct MyPlayer"));
        assert!(project
            .read("rust/src/my_player.rs")
            .contains("#[inherit(Node)]"));
        assert!(project
            .read("godot/native/MyPlayer.gdns")
            .contains("resource_name = \"MyPlayer\""));
        assert!(project
            .read("godot/native/MyPlayer.gdns")
            .contains("class_name = \"MyPlayer\""));
        assert!(project.read("rust/src/lib.rs").contains("mod my_player;"));
        assert!(project
            .read("rust/src/lib.rs")
            .contains("handle.add_class::<my_player::MyPlayer>();"));
    }

    #[test]
    fn test_process_ftw_command_build() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = util::get_current_platform().parse().unwrap();
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
        assert!(project
            .read("rust/Cargo.toml")
            .contains(&project.get_name()));
        assert!(project.exists(&format!(
            "lib/{}/{}{}.{}",
            target.to_cli_arg(),
            target.to_lib_prefix(),
            project.get_name(),
            target.to_lib_ext()
        )));
    }

    #[test]
    fn test_process_ftw_command_build_2x() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = util::get_current_platform().parse().unwrap();
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
        assert!(project
            .read("rust/Cargo.toml")
            .contains(&project.get_name()));
        assert!(project.exists(&format!(
            "lib/{}/{}{}.{}",
            target.to_cli_arg(),
            target.to_lib_prefix(),
            project.get_name(),
            target.to_lib_ext()
        )));
    }

    #[test]
    fn test_process_ftw_command_build_release() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = util::get_current_platform().parse().unwrap();
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Release,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
        assert!(project
            .read("rust/Cargo.toml")
            .contains(&project.get_name()));
        assert!(project.exists(&format!(
            "lib/{}/{}{}.{}",
            target.to_cli_arg(),
            target.to_lib_prefix(),
            project.get_name(),
            target.to_lib_ext()
        )));
    }

    #[test]
    fn test_process_ftw_command_export() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::Default,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = util::get_current_platform().parse().unwrap();
        let cmd = FtwCommand::Export {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../../"));
        assert!(project
            .read("rust/Cargo.toml")
            .contains(&project.get_name()));
        assert!(project.exists(&format!(
            "bin/{}/{}{}.{}",
            target.to_cli_arg(),
            target.to_lib_prefix(),
            project.get_name(),
            target.to_lib_ext()
        )));
        assert!(project.exists(&format!(
            "bin/{}/{}.debug.pck",
            target.to_cli_arg(),
            project.get_name()
        )));
        assert!(project.exists(&format!(
            "bin/{}/{}.debug.{}{}",
            target.to_cli_arg(),
            project.get_name(),
            target.to_cli_arg(),
            target.to_app_ext()
        )));
    }
}
