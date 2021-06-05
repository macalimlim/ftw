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
use std::{thread, time};

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
    thread::sleep(time::Duration::from_secs(2));
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
