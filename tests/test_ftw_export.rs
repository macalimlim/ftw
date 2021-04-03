mod common;

use assert_cmd::prelude::*;
use cargo_edit::get_crate_name_from_path;
use common::{ftw, generate_random_name, Project};
use ftw::ftw_target::FtwTarget;
use ftw::traits::{ToAppExt, ToCliArg, ToLibExt, ToLibPrefix};
use ftw::util::get_current_platform;
use predicates;
use predicates::prelude::*;
use std::{thread, time};

#[test]
fn test_ftw_export_no_target() {
    let name = generate_random_name();
    let project = Project::new(&name);
    ftw()
        .arg("new")
        .arg(&name)
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw().arg("export").current_dir(&name).assert().success();
    let current_platform = get_current_platform();
    let target: FtwTarget = current_platform.parse().unwrap();
    let crate_name = get_crate_name_from_path(&format!("{}/rust/", &name)).unwrap();
    thread::sleep(time::Duration::from_secs(2));
    assert!(project.read("rust/Cargo.toml").contains(&name));
    assert!(project.exists(&format!(
        "bin/{}/{}{}.{}",
        target.to_cli_arg(),
        target.to_lib_prefix(),
        name,
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
    drop(project);
}
