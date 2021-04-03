mod common;

use assert_cmd::prelude::*;
use common::{ftw, generate_random_name, Project};
use ftw::ftw_target::FtwTarget;
use ftw::traits::{ToCliArg, ToLibExt, ToLibPrefix};
use ftw::util::get_current_platform;
use predicates;
use predicates::prelude::*;
use std::{thread, time};

#[test]
fn test_ftw_build_no_target() {
    let name = generate_random_name();
    let project = Project::new(&name);
    ftw()
        .arg("new")
        .arg(&name)
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw().arg("build").current_dir(&name).assert().success();
    let current_platform = get_current_platform();
    let target: FtwTarget = current_platform.parse().unwrap();
    thread::sleep(time::Duration::from_secs(2));
    assert!(project.read("rust/Cargo.toml").contains(&name));
    assert!(project.exists(&format!(
        "lib/{}/{}{}.{}",
        target.to_cli_arg(),
        target.to_lib_prefix(),
        name,
        target.to_lib_ext()
    )));
    drop(project);
}
