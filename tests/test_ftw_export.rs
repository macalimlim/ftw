mod common;

use assert_cmd::prelude::*;
use common::ftw;
use ftw::ftw_target::FtwTarget;
use ftw::test_util::Project;
use ftw::traits::{ToAppExt, ToCliArg, ToLibExt, ToLibPrefix, ToStrTarget};
use ftw::type_alias::StrTarget;
use ftw::util::get_current_platform;
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_export_no_target() {
    let project = Project::new();
    let project_name = project.get_name();
    ftw()
        .arg("new")
        .arg(&project_name)
        .assert()
        .success()
        .stdout(predicates::str::contains("SUCCESS").from_utf8());
    ftw()
        .arg("export")
        .current_dir(&project_name)
        .assert()
        .success();
    let current_platform = get_current_platform();
    let target: FtwTarget = current_platform.parse().unwrap();
    assert!(project.read("rust/Cargo.toml").contains(&project_name));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let target_lib_ext = target.to_lib_ext();
    let target_app_ext = target.to_app_ext();
    assert!(project.exists(&format!(
        "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
    if target.is_linux() {
        assert!(project.exists(&format!(
            "bin/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
        )));
        assert!(project.exists(&format!(
            "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}.pck"
        )));
    }
    if target.is_windows() {
        assert!(project.exists(&format!(
            "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}.pck"
        )));
    }
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}{target_app_ext}"
    )));
}

#[test]
fn test_ftw_cross_export_multi_target() {
    let project = Project::new();
    let project_name = project.get_name();
    ftw()
        .arg("new")
        .arg(&project_name)
        .assert()
        .success()
        .stdout(predicates::str::contains("SUCCESS").from_utf8());
    let contents = r#"[ftw]
enable-cross-compilation=true
"#;
    let _ = project.create(".ftw", contents);
    assert!(project
        .read(".ftw")
        .contains("enable-cross-compilation=true"));
    let targets = vec![
        FtwTarget::MacOsAarch64,
        FtwTarget::LinuxX86_64,
        FtwTarget::MacOsX86_64,
        FtwTarget::IosAarch64,
        FtwTarget::WindowsX86_64Gnu,
    ];
    let str_targets = targets
        .iter()
        .map(|target| target.to_str_target())
        .collect::<Vec<StrTarget>>()
        .join(",");
    ftw()
        .arg("export")
        .arg(str_targets)
        .current_dir(&project_name)
        .assert()
        .success();
    assert!(project.read("rust/Cargo.toml").contains(&project_name));
    for target in targets {
        let target_cli_arg = target.to_cli_arg();
        let target_lib_prefix = target.to_lib_prefix();
        let target_lib_ext = target.to_lib_ext();
        let target_app_ext = target.to_app_ext();
        assert!(project.exists(&format!(
            "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
        )));
        if target.is_linux() {
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}.pck"
            )));
        }
        if target.is_windows() {
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}.pck"
            )));
        }
        if target.is_ios() {
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}.pck"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}.xcframework"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}.xcodeproj"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}"
            )));
        }
        if !target.is_ios() {
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{project_name}.debug.{target_cli_arg}{target_app_ext}"
            )));
        }
    }
    ftw()
        .arg("clean")
        .current_dir(&project_name)
        .assert()
        .success();
}
