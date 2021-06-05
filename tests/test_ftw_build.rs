mod common;

use assert_cmd::prelude::*;
use common::ftw;
use ftw::ftw_target::FtwTarget;
use ftw::test_util::Project;
use ftw::traits::{ToCliArg, ToLibExt, ToLibPrefix};
use ftw::util::get_current_platform;
use predicates;
use predicates::prelude::*;
use std::{thread, time};

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
    thread::sleep(time::Duration::from_secs(2));
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
