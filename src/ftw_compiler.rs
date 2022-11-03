use crate::ftw_build_type::FtwBuildType;
use crate::ftw_error::FtwError;
use crate::ftw_target::FtwTarget;
use crate::traits::{
    Compiler, Runner, ToAppExt, ToCliArg, ToExportArg, ToExportName, ToLibExt, ToLibPrefix,
};
use crate::util;
use cargo_edit::get_crate_name_from_path;
use command_macros::cmd;
use fs_extra::dir::CopyOptions;
use fs_extra::{move_items, remove_items};
use std::env;
use std::path::Path;

pub enum FtwCompiler {
    Local {
        target: FtwTarget,
        build_type: FtwBuildType,
    },
    Cross {
        target: FtwTarget,
        build_type: FtwBuildType,
    },
}

const DOCKER_IMAGE: &str = "macalimlim/godot-rust-cross-compiler:0.4.0";

#[rustfmt::skip::macros(cmd, format)]
impl Compiler for FtwCompiler {
    fn clean(&self) -> Result<(), FtwError> {
        match self {
            FtwCompiler::Local {
                target: _,
                build_type: _,
            } => cmd!(cargo clean).run(),
            FtwCompiler::Cross {
                target: _,
                build_type: _,
            } => {
                let volume_mount = format!("{}:/build", env::current_dir()?.display());
                cmd!(docker run ("-v") (volume_mount)
                     (DOCKER_IMAGE) ("/bin/bash") ("-c")
                     ("cargo clean ; rm -rf godot/.import"))
                .run()
            }
        }
    }

    fn build(&self) -> Result<(), FtwError> {
        match self {
            FtwCompiler::Local { target, build_type } => {
                let crate_name = get_crate_name_from_path("./rust/")?;
                let target_cli_arg = target.to_cli_arg();
                let build_type_cli_arg = build_type.to_cli_arg();
                let target_lib_ext = target.to_lib_ext();
                let build_type_string = build_type.to_string().to_lowercase();
                let target_lib_prefix = target.to_lib_prefix();
                let source_path = format!(
                    "./target/{}/{}/{}{}.{}",
                    &target_cli_arg,
                    build_type_string,
                    target_lib_prefix,
                    crate_name,
                    &target_lib_ext
                );
                let target_path = format!("./lib/{}", &target_cli_arg);
                cmd!(cargo build ("--target") (target_cli_arg) if (build_type.is_release()) { (build_type_cli_arg) }).run()?;
                let lib = format!(
                    "{}/{}{}.{}",
                    target_path, target_lib_prefix, crate_name, target_lib_ext
                );
                if Path::new(&lib).exists() {
                    let target_lib_files = vec![lib];
                    remove_items(&target_lib_files)?;
                }
                let options = CopyOptions::new();
                let source_paths = vec![source_path];
                move_items(&source_paths, target_path, &options)?;
                Ok(())
            }
            FtwCompiler::Cross { target, build_type } => {
                let target_cli_arg = target.to_cli_arg();
                let build_type_cli_arg = build_type.to_cli_arg();
                let cargo_build_cmd = format!(
                    "cargo build --target {} {} ; mv -b ./target/{}/{}/*.{} ./lib/{}",
                    target_cli_arg,
                    build_type_cli_arg,
                    target_cli_arg,
                    build_type,
                    target.to_lib_ext(),
                    target_cli_arg
                );
                let volume_mount = format!("{}:/build", env::current_dir()?.display());
                cmd!(docker run ("-v") (volume_mount)
                     if (target == &FtwTarget::WindowsX86_64Gnu || target == &FtwTarget::WindowsX86_64Msvc) {("-e") ("C_INCLUDE_PATH=/usr/x86_64-w64-mingw32/include")}
                     if (target == &FtwTarget::MacOsX86_64) {("-e") ("CC=/opt/macosx-build-tools/cross-compiler/bin/x86_64-apple-darwin14-cc") ("-e") ("C_INCLUDE_PATH=/opt/macosx-build-tools/cross-compiler/SDK/MacOSX10.10.sdk/usr/include")}
                     (DOCKER_IMAGE) ("/bin/bash") ("-c")
                     (cargo_build_cmd)).run()
            }
        }
    }

    fn export(&self) -> Result<(), FtwError> {
        match self {
            FtwCompiler::Local { target, build_type } => {
                let crate_name = get_crate_name_from_path("./rust/")?;
                let target_cli_arg = target.to_cli_arg();
                let target_export_name = target.to_export_name();
                let build_type_export_arg = build_type.to_export_arg();
                let build_type = build_type.to_string().to_lowercase();
                let target_app_ext = target.to_app_ext();
                let export_name =
                    format!("{}.{}.{}", target_export_name, target_cli_arg, build_type);
                let export_path = format!(
                    "../bin/{}/{}.{}.{}{}",
                    &target_cli_arg, &crate_name, build_type, &target_cli_arg, &target_app_ext
                );
                let current_platform = util::get_current_platform().parse().unwrap_or_default();
                let godot_executable = util::get_godot_exe_for_exporting(current_platform);
                env::set_current_dir(Path::new("./godot"))?;
                cmd!((godot_executable.as_str())(build_type_export_arg)(export_name)(export_path))
                    .run()
            }
            FtwCompiler::Cross { target, build_type } => {
                let crate_name = get_crate_name_from_path("./rust/")?;
                let target_cli_arg = target.to_cli_arg();
                let target_export_name = target.to_export_name();
                let build_type = build_type.to_string().to_lowercase();
                let target_app_ext = target.to_app_ext();
                let export_name =
                    format!("{}.{}.{}", target_export_name, target_cli_arg, build_type);
                let export_path = format!(
                    "../bin/{}/{}.{}.{}{}",
                    &target_cli_arg, &crate_name, build_type, &target_cli_arg, &target_app_ext
                );
                let godot_export_cmd =
                    format!("cd godot/ ; godot_headless --export '{}' {}", export_name, export_path);
                let volume_mount = format!("{}:/build", env::current_dir()?.display());
                cmd!(docker run ("-v") (volume_mount)
                    (DOCKER_IMAGE) ("/bin/bash") ("-c")
                    (godot_export_cmd))
                .run()
            }
        }
    }
}
