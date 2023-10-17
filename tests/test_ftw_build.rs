mod common;

use assert_cmd::prelude::*;
use common::ftw;
use ftw::ftw_target::FtwTarget;
use ftw::test_util::Project;
use ftw::traits::{ToCliArg, ToLibExt, ToLibPrefix};
use ftw::util::get_current_platform;
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_build_no_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("build")
        .current_dir(&project.get_name())
        .assert()
        .success();
    let current_platform = get_current_platform();
    let target: FtwTarget = current_platform.parse().unwrap();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let project_name = project.get_name();
    let target_lib_ext = target.to_lib_ext();
    assert!(project.exists(&format!(
        "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
}

#[test]
fn test_ftw_cross_build_linux_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    let contents = r#"[ftw]
enable-cross-compilation=true
"#;
    let _ = project.create(".ftw", contents);
    assert!(project
        .read(".ftw")
        .contains("enable-cross-compilation=true"));
    let target = FtwTarget::LinuxX86_64;
    ftw()
        .arg("build")
        .arg("linux-x86_64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let project_name = project.get_name();
    let target_lib_ext = target.to_lib_ext();
    assert!(project.exists(&format!(
        "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
}

#[test]
fn test_ftw_cross_build_windows_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    let contents = r#"[ftw]
enable-cross-compilation=true
"#;
    let _ = project.create(".ftw", contents);
    assert!(project
        .read(".ftw")
        .contains("enable-cross-compilation=true"));
    let target = FtwTarget::WindowsX86_64Gnu;
    ftw()
        .arg("build")
        .arg("windows-x86_64-gnu")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let project_name = project.get_name();
    let target_lib_ext = target.to_lib_ext();
    assert!(project.exists(&format!(
        "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
}

#[test]
fn test_ftw_cross_build_macos_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    let contents = r#"[ftw]
enable-cross-compilation=true
"#;
    let _ = project.create(".ftw", contents);
    assert!(project
        .read(".ftw")
        .contains("enable-cross-compilation=true"));
    let target = FtwTarget::MacOsX86_64;
    ftw()
        .arg("build")
        .arg("macos-x86_64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let project_name = project.get_name();
    let target_lib_ext = target.to_lib_ext();
    assert!(project.exists(&format!(
        "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
}

#[test]
fn test_ftw_cross_build_android_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    let contents = r#"[ftw]
enable-cross-compilation=true
"#;
    let _ = project.create(".ftw", contents);
    assert!(project
        .read(".ftw")
        .contains("enable-cross-compilation=true"));
    let target = FtwTarget::AndroidLinuxAarch64;
    ftw()
        .arg("build")
        .arg("android-aarch64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let project_name = project.get_name();
    let target_lib_ext = target.to_lib_ext();
    assert!(project.exists(&format!(
        "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
}

#[test]
fn test_ftw_cross_build_ios_target() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    let contents = r#"[ftw]
enable-cross-compilation=true
"#;
    let _ = project.create(".ftw", contents);
    assert!(project
        .read(".ftw")
        .contains("enable-cross-compilation=true"));
    let target = FtwTarget::IosAarch64;
    ftw()
        .arg("build")
        .arg("ios-aarch64")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    let target_cli_arg = target.to_cli_arg();
    let target_lib_prefix = target.to_lib_prefix();
    let project_name = project.get_name();
    let target_lib_ext = target.to_lib_ext();
    assert!(project.exists(&format!(
        "lib/{target_cli_arg}/{target_lib_prefix}{project_name}.{target_lib_ext}"
    )));
    ftw()
        .arg("clean")
        .current_dir(&project.get_name())
        .assert()
        .success();
}
