mod common;

use assert_cmd::prelude::*;
use common::ftw;
use ftw::ftw_target::FtwTarget;
use ftw::test_util::Project;
use ftw::traits::{ToAppExt, ToCliArg, ToLibExt, ToLibPrefix};
use ftw::util::{get_crate_name_from_path, get_current_platform};
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_export_no_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("SUCCESS").from_utf8());
    ftw()
        .arg("export")
        .current_dir(&project.get_name())
        .assert()
        .success();
    let current_platform = get_current_platform();
    let target: FtwTarget = current_platform.parse().unwrap();
    let project_name = project.get_name();
    let crate_name = get_crate_name_from_path(&format!("{project_name}/rust/")).unwrap();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let target_lib_ext = target.to_lib_ext();
    let target_app_ext = target.to_app_ext();
    if target == FtwTarget::LinuxX86_64 {
        assert!(project.exists(&format!(
            "bin/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
        )));
        assert!(project.exists(&format!(
            "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.pck"
        )));
    }
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}{target_app_ext}"
    )));
}

#[test]
fn test_ftw_cross_export_linux_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
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
    let target = FtwTarget::LinuxX86_64;
    ftw()
        .arg("export")
        .arg("linux-x86_64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
    let project_name = project.get_name();
    let crate_name = get_crate_name_from_path(&format!("{project_name}/rust/")).unwrap();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let target_lib_ext = target.to_lib_ext();
    let target_app_ext = target.to_app_ext();
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.pck"
    )));
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}{target_app_ext}"
    )));
}

#[test]
fn test_ftw_cross_export_macos_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
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
    let target = FtwTarget::MacOsX86_64;
    ftw()
        .arg("export")
        .arg("macos-x86_64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
    let project_name = project.get_name();
    let crate_name = get_crate_name_from_path(&format!("{project_name}/rust/")).unwrap();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_app_ext = target.to_app_ext();
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}{target_app_ext}"
    )));
}

#[test]
fn test_ftw_cross_export_macos_arm_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
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
    let target = FtwTarget::MacOsAarch64;
    ftw()
        .arg("export")
        .arg("macos-aarch64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
    let project_name = project.get_name();
    let crate_name = get_crate_name_from_path(&format!("{project_name}/rust/")).unwrap();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_app_ext = target.to_app_ext();
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}{target_app_ext}"
    )));
}

#[test]
fn test_ftw_cross_export_ios_target() {
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
    let target = FtwTarget::IosAarch64;
    ftw()
        .arg("export")
        .arg("ios-aarch64")
        .current_dir(&project_name)
        .assert()
        .success();
    ftw()
        .arg("clean")
        .current_dir(&project_name)
        .assert()
        .success();
    let crate_name = get_crate_name_from_path(&format!("{project_name}/rust/")).unwrap();
    assert!(project.read("rust/Cargo.toml").contains(&project_name));
    let target_cli_arg = target.to_cli_arg();
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.pck"
    )));
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.xcframework"
    )));
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.xcodeproj"
    )));
    assert!(project.exists(&format!(
        "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}"
    )));
}

#[test]
fn test_ftw_cross_export_multi_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
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
    ftw()
        .arg("export")
        .arg("macos-aarch64,linux-x86_64,macos-x86_64,ios-aarch64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
    let project_name = project.get_name();
    let crate_name = get_crate_name_from_path(&format!("{project_name}/rust/")).unwrap();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let targets = vec![
        FtwTarget::MacOsAarch64,
        FtwTarget::LinuxX86_64,
        FtwTarget::MacOsX86_64,
        FtwTarget::IosAarch64,
    ];
    for target in targets {
        let target_cli_arg = target.to_cli_arg();
        let target_lib_prefix = target.to_lib_prefix();
        let target_lib_ext = target.to_lib_ext();
        let target_app_ext = target.to_app_ext();
        if target == FtwTarget::LinuxX86_64 {
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.pck"
            )));
        }
        if target == FtwTarget::IosAarch64 {
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.pck"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.xcframework"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}.xcodeproj"
            )));
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}"
            )));
        }
        if target != FtwTarget::IosAarch64 {
            assert!(project.exists(&format!(
                "bin/{target_cli_arg}/{crate_name}.debug.{target_cli_arg}{target_app_ext}"
            )));
        }
    }
}
