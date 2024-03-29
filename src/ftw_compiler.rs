use crate::ftw_build_type::FtwBuildType;
use crate::ftw_error::FtwError;
use crate::ftw_target::FtwTarget;
use crate::traits::{
    Compiler, Runner, ToAppExt, ToCliArg, ToExportArg, ToExportName, ToLibExt, ToLibPrefix,
};
use crate::util;
use command_macros::cmd;
use fs_extra::dir::CopyOptions;
use fs_extra::{move_items, remove_items};
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

const DOCKER_IMAGE: &str = "macalimlim/godot-rust-cross-compiler:0.8.0";
const MACOSX_CROSS_COMPILER_PATH: &str = "/opt/macosx-build-tools/cross-compiler";
const MIN_MACOSX_SDK_VERSION: &str = "11.3";
const MIN_OSXCROSS_TARGET_VERSION: &str = "20.4";
const IOS_CROSS_COMPILER_PATH: &str = "/opt/ios-build-tools/cross-compiler";
const MIN_IOS_SDK_VERSION: &str = "14.5";
const SHELL: &str = "/bin/bash";

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
                let current_dir = Path::new(".").canonicalize()?;
                let current_dir_display = current_dir.display();
                let volume_mount = format!("{current_dir_display}:/build");
                cmd!(docker run ("-v") (volume_mount)
                     (DOCKER_IMAGE) (SHELL) ("-c")
                     ("cargo clean ; rm -rf godot/.import"))
                .run()
            }
        }
    }

    fn build(&self) -> Result<(), FtwError> {
        match self {
            FtwCompiler::Local { target, build_type } => {
                let crate_name = util::get_crate_name_from_path("./rust/")?;
                let target_cli_arg = target.to_cli_arg();
                let build_type_cli_arg = build_type.to_cli_arg();
                let target_lib_ext = target.to_lib_ext();
                let build_type_string = build_type.to_string().to_lowercase();
                let target_lib_prefix = target.to_lib_prefix();
                let source_path = format!("./target/{target_cli_arg}/{build_type_string}/{target_lib_prefix}{crate_name}.{target_lib_ext}");
                let target_path = format!("./lib/{target_cli_arg}");
                cmd!(cargo build ("--target") (target_cli_arg) if (build_type.is_release()) { (build_type_cli_arg) }).run()?;
                let lib = format!("{target_path}/{target_lib_prefix}{crate_name}.{target_lib_ext}");
                if Path::new(&lib).exists() {
                    let target_lib_files = [lib];
                    remove_items(&target_lib_files)?;
                }
                let options = CopyOptions::new();
                let source_paths = [source_path];
                move_items(&source_paths, target_path, &options)?;
                Ok(())
            }
            FtwCompiler::Cross { target, build_type } => {
                let target_cli_arg = target.to_cli_arg();
                let build_type_cli_arg = build_type.to_cli_arg();
                let target_lib_ext = target.to_lib_ext();
                let cargo_build_cmd = format!("cargo build --target {target_cli_arg} {build_type_cli_arg} ; mv -b ./target/{target_cli_arg}/{build_type}/*.{target_lib_ext} ./lib/{target_cli_arg}");
                let current_dir = Path::new(".").canonicalize()?;
                let current_dir_display = current_dir.display();
                let volume_mount = format!("{current_dir_display}:/build");
                let macosx_sdk_version_output = cmd!(docker run (DOCKER_IMAGE) ("/bin/bash") ("-c") ("echo $MACOSX_SDK_VERSION"))
                    .output()?;
                let macosx_sdk_version = String::from_utf8(macosx_sdk_version_output.stdout)
                    .unwrap_or(String::from(MIN_MACOSX_SDK_VERSION));
                let macosx_sdk_version = macosx_sdk_version.trim();
                let macosx_c_include_path = format!("C_INCLUDE_PATH={MACOSX_CROSS_COMPILER_PATH}/SDK/MacOSX{macosx_sdk_version}.sdk/usr/include");
                let osxcross_target_output = cmd!(docker run (DOCKER_IMAGE) (SHELL) ("-c") ("{MACOSX_CROSS_COMPILER_PATH}/bin/osxcross-conf | grep OSXCROSS_TARGET= | sed 's/export OSXCROSS_TARGET=darwin//g'"))
                    .output()?;
                let osxcross_target_version = String::from_utf8(osxcross_target_output.stdout)
                    .unwrap_or(String::from(MIN_OSXCROSS_TARGET_VERSION));
                let macosx_cc = format!("CC={MACOSX_CROSS_COMPILER_PATH}/bin/{target_cli_arg}{osxcross_target_version}-cc");
                let ios_sdk_version_output =
                    cmd!(docker run (DOCKER_IMAGE) ("/bin/bash") ("-c") ("echo $IOS_SDK_VERSION"))
                        .output()?;
                let ios_sdk_version = String::from_utf8(ios_sdk_version_output.stdout)
                    .unwrap_or(String::from(MIN_IOS_SDK_VERSION));
                let ios_sdk_version = ios_sdk_version.trim();
                let ios_c_include_path = format!("C_INCLUDE_PATH={IOS_CROSS_COMPILER_PATH}/SDK/iPhoneOS{ios_sdk_version}.sdk/usr/include");
                let ios_ld_library_path = format!("LD_LIBRARY_PATH={IOS_CROSS_COMPILER_PATH}/lib");
                cmd!(docker run ("-v") (volume_mount)
                     if (target == &FtwTarget::WindowsX86_64Gnu || target == &FtwTarget::WindowsX86_64Msvc) {("-e") ("C_INCLUDE_PATH=/usr/x86_64-w64-mingw32/include")}
                     if (target == &FtwTarget::MacOsAarch64 || target == &FtwTarget::MacOsX86_64) {("-e") (macosx_cc) ("-e") (macosx_c_include_path)}
                     if (target == &FtwTarget::IosAarch64) {("-e") (ios_c_include_path) ("-e") (ios_ld_library_path)}
                     (DOCKER_IMAGE) (SHELL) ("-c")
                     (cargo_build_cmd)).run()
            }
        }
    }

    fn export(&self) -> Result<(), FtwError> {
        match self {
            FtwCompiler::Local { target, build_type } => {
                let crate_name = util::get_crate_name_from_path("./rust/")?;
                let target_cli_arg = target.to_cli_arg();
                let target_export_name = target.to_export_name();
                let build_type_export_arg = build_type.to_export_arg();
                let build_type = build_type.to_string().to_lowercase();
                let target_app_ext = target.to_app_ext();
                let export_name = format!("{target_export_name}.{target_cli_arg}.{build_type}");
                let export_path = format!("../bin/{target_cli_arg}/{crate_name}.{build_type}.{target_cli_arg}{target_app_ext}");
                let current_platform = util::get_current_platform().parse().unwrap_or_default();
                let godot_executable = util::get_godot_exe_for_exporting(current_platform);
                cmd!((godot_executable.as_str())(build_type_export_arg)(export_name)(export_path))
                    .current_dir("./godot")
                    .run()
            }
            FtwCompiler::Cross { target, build_type } => {
                let crate_name = util::get_crate_name_from_path("./rust/")?;
                let target_cli_arg = target.to_cli_arg();
                let target_export_name = target.to_export_name();
                let build_type = build_type.to_string().to_lowercase();
                let target_app_ext = target.to_app_ext();
                let export_name = format!("{target_export_name}.{target_cli_arg}.{build_type}");
                let export_path = format!("../bin/{target_cli_arg}/{crate_name}.{build_type}.{target_cli_arg}{target_app_ext}");
                let godot_export_cmd =
                    format!("cd godot/ ; godot_headless --export '{export_name}' {export_path}");
                let current_dir = Path::new(".").canonicalize()?;
                let current_dir_display = current_dir.display();
                let volume_mount = format!("{current_dir_display}:/build");
                cmd!(docker run ("-v") (volume_mount)
                    (DOCKER_IMAGE) (SHELL) ("-c")
                    (godot_export_cmd))
                .run()
            }
        }
    }
}
