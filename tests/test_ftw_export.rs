mod common;

use assert_cmd::prelude::*;
use cargo_edit::get_crate_name_from_path;
use common::ftw;
use ftw::ftw_target::FtwTarget;
use ftw::test_util::Project;
use ftw::traits::{ToAppExt, ToCliArg, ToLibExt, ToLibPrefix};
use ftw::util::get_current_platform;
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
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("export")
        .current_dir(&project.get_name())
        .assert()
        .success();
    let current_platform = get_current_platform();
    let target: FtwTarget = current_platform.parse().unwrap();
    let crate_name = get_crate_name_from_path(&format!("{}/rust/", &project.get_name())).unwrap();
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
        crate_name
    )));
    assert!(project.exists(&format!(
        "bin/{}/{}.debug.{}{}",
        target.to_cli_arg(),
        crate_name,
        target.to_cli_arg(),
        target.to_app_ext()
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
    let crate_name = get_crate_name_from_path(&format!("{}/rust/", &project.get_name())).unwrap();
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
        crate_name
    )));
    assert!(project.exists(&format!(
        "bin/{}/{}.debug.{}{}",
        target.to_cli_arg(),
        crate_name,
        target.to_cli_arg(),
        target.to_app_ext()
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
    let crate_name = get_crate_name_from_path(&format!("{}/rust/", &project.get_name())).unwrap();
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
    assert!(project.exists(&format!(
        "bin/{}/{}.debug.{}{}",
        target.to_cli_arg(),
        crate_name,
        target.to_cli_arg(),
        target.to_app_ext()
    )));
}
