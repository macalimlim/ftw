use crate::ftw_build_type::FtwBuildType;
use crate::ftw_configuration::FtwConfiguration;
use crate::ftw_error::FtwError;
use crate::ftw_machine_type::FtwMachineType;
use crate::ftw_node_type::FtwNodeType;
use crate::ftw_success::FtwSuccess;
use crate::ftw_target::FtwTarget;
use crate::ftw_template::FtwTemplate;
use crate::traits::{Compiler, Processor, Runner, ToCliArg, ToGitUrl};
use crate::type_alias::{ClassName, FtwResult, ProjectName};
use crate::util;

use cargo_generate::{generate, Args, Vcs};
use command_macros::cmd;
use fs_extra::remove_items;
use kstring::KStringBase;
use liquid::{object, Object, ParserBuilder};
use liquid_core::model::{ScalarCow, Value};
use regex::Regex;
use std::fs::{create_dir_all, read_dir, write, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use strum::IntoEnumIterator;
use voca_rs::Voca;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq)]
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
    Clean,
}

#[rustfmt::skip::macros(cmd, format)]
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
            bin: false,
            lib: true,
            ssh_identity: None,
            subfolder: None,
            define: vec![],
            init: false,
            path: None,
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

    fn is_valid_project() -> Result<bool, FtwError> {
        let project_files = vec![
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
        let is_valid_project = project_files.iter().all(|i| {
            // TODO: Remove the check for the Makefile in the future
            if i == &"Makefile" {
                Path::new(i).exists() || Path::new("Makefile.toml").exists()
            } else {
                Path::new(i).exists()
            }
        });
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
        Ok(reg_ex.find(contents).is_some())
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
                    let replaced_path_display = path_display.as_str().replace('\\', "/");
                    let module_name_vec: Vec<&str> = replaced_path_display.split('/').collect();
                    let (_, module_path) = module_name_vec.split_at(2);
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
        object!({ "class_name": class_name, "node_type": node_type.to_string() })
    }

    fn create_lib_rs_file(class_name: &str, node_type: FtwNodeType) -> Result<(), FtwError> {
        let mut tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
        let modules = FtwCommand::get_modules_from_directory("rust/src")?;
        let k = KStringBase::from_ref("modules");
        let v = Value::Scalar(ScalarCow::from(modules));
        tmpl_globals.insert(k, v);
        let classes = FtwCommand::get_classes_from_directory("rust/src")?;
        let k = KStringBase::from_ref("classes");
        let v = Value::Scalar(ScalarCow::from(classes));
        tmpl_globals.insert(k, v);
        let template = &String::from_utf8_lossy(include_bytes!("templates/lib_tmpl.rs"));
        FtwCommand::create_file(template, "rust/src/lib.rs", &tmpl_globals)
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
                    let module_path = path
                        .file_stem()
                        .ok_or(FtwError::PathError)?
                        .to_os_string()
                        .to_str()
                        .ok_or(FtwError::StringConversionError)?
                        .to_string();
                    modules.push(module_path);
                }
            }
            let is_dir = path.is_dir();
            let mod_rs_file = format!("{}/mod.rs", entry_path.as_path().display());
            let mod_rs_file_path = Path::new(&mod_rs_file);
            let is_contains_mod_rs = mod_rs_file_path.exists();
            if is_dir && is_contains_mod_rs {
                let module_path = path
                    .file_name()
                    .ok_or(FtwError::PathError)?
                    .to_str()
                    .ok_or(FtwError::StringConversionError)?
                    .to_string();
                modules.push(module_path);
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
            let tmpl_globals = object!({ "modules": modules });
            let template = &String::from_utf8_lossy(include_bytes!("templates/mod_tmpl.rs"));
            FtwCommand::create_file(template, &mod_rs_file, &tmpl_globals)?;
            match directories.split_last() {
                Some((_, init)) => FtwCommand::create_mod_rs_file(base_src_path, init),
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
        let src_dir_path = FtwCommand::create_directory(base_src_path, directories)?;
        let class_rs_file = format!("{}/{}.rs", &src_dir_path, class_name._snake_case());
        if !Path::new(&class_rs_file).exists() {
            let tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
            let template = &String::from_utf8_lossy(include_bytes!("templates/class_tmpl.rs"));
            FtwCommand::create_file(template, &class_rs_file, &tmpl_globals)?;
        }
        FtwCommand::create_mod_rs_file(base_src_path, directories)?;
        Ok(())
    }

    fn create_gdns_file(
        class_name: &str,
        directories: &[String],
        node_type: FtwNodeType,
    ) -> Result<(), FtwError> {
        let gdns_dir_path = FtwCommand::create_directory("godot/native", directories)?;
        let gdns_file = format!("{}/{}.gdns", &gdns_dir_path, class_name._pascal_case());
        if !Path::new(&gdns_file).exists() {
            let tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
            let template = &String::from_utf8_lossy(include_bytes!("templates/gdns_tmpl.gdns"));
            FtwCommand::create_file(template, &gdns_file, &tmpl_globals)?;
        }
        Ok(())
    }

    fn create_tscn_file(
        class_name: &str,
        directories: &[String],
        node_type: FtwNodeType,
    ) -> Result<(), FtwError> {
        let tscn_dir_path = FtwCommand::create_directory("godot/scenes", directories)?;
        let tscn_file = format!("{}/{}.tscn", &tscn_dir_path, class_name._pascal_case());
        if !Path::new(&tscn_file).exists() {
            let mut tmpl_globals = FtwCommand::get_tmpl_globals(class_name, node_type);
            let k = KStringBase::from_ref("dir_path");
            let v = Value::Scalar(ScalarCow::from(if directories.is_empty() {
                "".to_string()
            } else {
                let mut dir = directories.join("/");
                dir.push('/');
                dir
            }));
            tmpl_globals.insert(k, v);
            let template = &String::from_utf8_lossy(include_bytes!("templates/tscn_tmpl.tscn"));
            FtwCommand::create_file(template, &tscn_file, &tmpl_globals)?;
        }
        Ok(())
    }

    fn clean() -> Result<(), FtwError> {
        let compiler =
            FtwConfiguration::new().get_compiler(FtwTarget::default(), FtwBuildType::default());
        compiler.clean()
    }

    fn build_lib(target: FtwTarget, build_type: FtwBuildType) -> Result<(), FtwError> {
        let compiler = FtwConfiguration::new().get_compiler(target, build_type);
        compiler.build()
    }

    fn export_game(target: FtwTarget, build_type: FtwBuildType) -> Result<(), FtwError> {
        let compiler = FtwConfiguration::new().get_compiler(target, build_type);
        compiler.export()
    }

    fn run_with_godot(machine_type: &FtwMachineType) -> Result<(), FtwError> {
        let godot_executable = util::get_godot_exe_for_running(machine_type);
        cmd!((godot_executable) ("--path") ("godot/") if (machine_type.is_desktop()) { (machine_type.to_cli_arg()) }).run()
    }
}

#[rustfmt::skip]
impl Processor for FtwCommand {
    fn process(&self) -> FtwResult {
        match self {
            FtwCommand::New { project_name, template } => {
                FtwCommand::generate_project(project_name, template)?;
                FtwCommand::append_to_gitignore(project_name)?;
                FtwCommand::delete_items(project_name)?;
                let project_name = project_name.to_string();
                Ok(FtwSuccess::New { project_name, template })
            }
            FtwCommand::Class { class_name, node_type } => {
                FtwCommand::is_valid_project()?;
                let (class_name, directories) = util::get_class_name_and_directories(class_name);
                FtwCommand::create_class_rs_file(&class_name, &directories, *node_type)?;
                FtwCommand::create_gdns_file(&class_name, &directories, *node_type)?;
                FtwCommand::create_tscn_file(&class_name, &directories, *node_type)?;
                FtwCommand::create_lib_rs_file(&class_name, *node_type)?;
                Ok(FtwSuccess::Class { class_name, node_type })
            }
            FtwCommand::Singleton { class_name } => {
                FtwCommand::is_valid_project()?;
                let node_type = FtwNodeType::default();
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
                let build_type = FtwBuildType::default();
                let current_platform = util::get_current_platform();
                let target: FtwTarget = current_platform.parse().unwrap_or_default();
                if machine_type.is_server() {
                    target.is_linux_server()?;
                }
                FtwCommand::build_lib(target, build_type)?;
                FtwCommand::run_with_godot(machine_type)?;
                Ok(FtwSuccess::Run { machine_type })
            }
            FtwCommand::Build { target, build_type } => {
                FtwCommand::is_valid_project()?;
                FtwCommand::clean()?;
                FtwCommand::build_lib(*target, *build_type)?;
                Ok(FtwSuccess::Build { target, build_type })
            }
            FtwCommand::Export { target, build_type } => {
                FtwCommand::is_valid_project()?;
                FtwCommand::clean()?;
                FtwCommand::build_lib(*target, *build_type)?;
                FtwCommand::export_game(*target, *build_type)?;
                Ok(FtwSuccess::Export { target, build_type })
            }
            FtwCommand::Clean => {
                FtwCommand::clean()?;
                Ok(FtwSuccess::Clean)
            }
        }
    }
}

#[cfg(test)]
mod ftw_command_tests {
    use super::*;

    use crate::{
        test_util::Project,
        traits::{ToAppExt, ToLibExt, ToLibPrefix},
    };
    use std::env;

    #[test]
    fn test_is_valid_project_no_cargo_toml() {
        let project = Project::default();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
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
            template: FtwTemplate::default(),
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
            template: FtwTemplate::default(),
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
            template: FtwTemplate::default(),
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
            template: FtwTemplate::default(),
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
            template: FtwTemplate::default(),
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
    fn test_process_ftw_command_cross_build_linux_target() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
        };
        let _ = cmd.process();
        let contents = r#"[ftw]
enable-cross-compilation=true
"#;
        let _ = project.create(".ftw", contents);
        assert!(project
            .read(".ftw")
            .contains("enable-cross-compilation=true"));
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = FtwTarget::LinuxX86_64;
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let cmd = FtwCommand::Clean;
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
    fn test_process_ftw_command_cross_build_windows_target() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
        };
        let _ = cmd.process();
        let contents = r#"[ftw]
enable-cross-compilation=true
"#;
        let _ = project.create(".ftw", contents);
        assert!(project
            .read(".ftw")
            .contains("enable-cross-compilation=true"));
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = FtwTarget::WindowsX86_64Gnu;
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let cmd = FtwCommand::Clean;
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
    fn test_process_ftw_command_cross_build_macos_target() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
        };
        let _ = cmd.process();
        let contents = r#"[ftw]
enable-cross-compilation=true
"#;
        let _ = project.create(".ftw", contents);
        assert!(project
            .read(".ftw")
            .contains("enable-cross-compilation=true"));
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = FtwTarget::MacOsX86_64;
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let cmd = FtwCommand::Clean;
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
    fn test_process_ftw_command_cross_build_android_target() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
        };
        let _ = cmd.process();
        let contents = r#"[ftw]
enable-cross-compilation=true
"#;
        let _ = project.create(".ftw", contents);
        assert!(project
            .read(".ftw")
            .contains("enable-cross-compilation=true"));
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = FtwTarget::AndroidLinuxAarch64;
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let cmd = FtwCommand::Clean;
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
    fn test_process_ftw_command_cross_build_ios_target() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
        };
        let _ = cmd.process();
        let contents = r#"[ftw]
enable-cross-compilation=true
"#;
        let _ = project.create(".ftw", contents);
        assert!(project
            .read(".ftw")
            .contains("enable-cross-compilation=true"));
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = FtwTarget::IosAarch64;
        let cmd = FtwCommand::Build {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let cmd = FtwCommand::Clean;
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
            template: FtwTemplate::default(),
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
            template: FtwTemplate::default(),
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
            template: FtwTemplate::default(),
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
            "bin/{}/{}.debug.{}.pck",
            target.to_cli_arg(),
            project.get_name(),
            target.to_cli_arg()
        )));
        assert!(project.exists(&format!(
            "bin/{}/{}.debug.{}{}",
            target.to_cli_arg(),
            project.get_name(),
            target.to_cli_arg(),
            target.to_app_ext()
        )));
    }

    #[test]
    fn test_process_ftw_command_cross_export_linux_target() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
        };
        let _ = cmd.process();
        let contents = r#"[ftw]
enable-cross-compilation=true
"#;
        let _ = project.create(".ftw", contents);
        assert!(project
            .read(".ftw")
            .contains("enable-cross-compilation=true"));
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = FtwTarget::LinuxX86_64;
        let cmd = FtwCommand::Export {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let cmd = FtwCommand::Clean;
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
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
            "bin/{}/{}.debug.{}.pck",
            target.to_cli_arg(),
            project.get_name(),
            target.to_cli_arg()
        )));
        assert!(project.exists(&format!(
            "bin/{}/{}.debug.{}{}",
            target.to_cli_arg(),
            project.get_name(),
            target.to_cli_arg(),
            target.to_app_ext()
        )));
    }

    #[test]
    fn test_process_ftw_command_cross_export_macos_target() {
        let project = Project::new();
        let cmd = FtwCommand::New {
            project_name: project.get_name(),
            template: FtwTemplate::default(),
        };
        let _ = cmd.process();
        let contents = r#"[ftw]
enable-cross-compilation=true
"#;
        let _ = project.create(".ftw", contents);
        assert!(project
            .read(".ftw")
            .contains("enable-cross-compilation=true"));
        let _ = env::set_current_dir(Path::new(&project.get_name()));
        let target: FtwTarget = FtwTarget::MacOsX86_64;
        let cmd = FtwCommand::Export {
            target: target.clone(),
            build_type: FtwBuildType::Debug,
        };
        let _ = cmd.process();
        let cmd = FtwCommand::Clean;
        let _ = cmd.process();
        let _ = env::set_current_dir(Path::new("../"));
        assert!(project
            .read("rust/Cargo.toml")
            .contains(&project.get_name()));
        assert!(project.exists(&format!(
            "bin/{}/{}.debug.{}{}",
            target.to_cli_arg(),
            project.get_name(),
            target.to_cli_arg(),
            target.to_app_ext()
        )));
    }
}
