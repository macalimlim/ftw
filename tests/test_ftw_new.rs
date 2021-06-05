mod common;

use assert_cmd::prelude::*;
use common::ftw;
use ftw::test_util::Project;
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_new() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .arg("default")
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    assert!(project.exists(".gitignore"));
    assert!(project.exists("Cargo.toml"));
    assert!(project.exists("Makefile"));
    assert!(project.exists("godot/default_env.tres"));
    assert!(project.exists("godot/export_presets.cfg"));
    assert!(project.exists("godot/native/game.gdnlib"));
    assert!(project.exists("godot/project.godot"));
    assert!(project.exists("rust/Cargo.toml"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(!project.exists("LICENSE"));
    assert!(!project.exists(".travis.yml"));
    assert!(!project.exists("sh"));
    assert!(project.read(".gitignore").contains(".ftw"));
    assert!(project.read(".gitignore").contains("bin/*"));
    assert!(project.read(".gitignore").contains("export_presets.cfg"));
    assert!(project.read(".gitignore").contains("lib/*"));
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
}

#[test]
fn test_ftw_new_no_template() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    assert!(project.exists(".gitignore"));
    assert!(project.exists("Cargo.toml"));
    assert!(project.exists("Makefile"));
    assert!(project.exists("godot/default_env.tres"));
    assert!(project.exists("godot/export_presets.cfg"));
    assert!(project.exists("godot/native/game.gdnlib"));
    assert!(project.exists("godot/project.godot"));
    assert!(project.exists("rust/Cargo.toml"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(!project.exists("LICENSE"));
    assert!(!project.exists(".travis.yml"));
    assert!(!project.exists("sh"));
    assert!(project.read(".gitignore").contains(".ftw"));
    assert!(project.read(".gitignore").contains("bin/*"));
    assert!(project.read(".gitignore").contains("export_presets.cfg"));
    assert!(project.read(".gitignore").contains("lib/*"));
    assert!(project
        .read("rust/Cargo.toml")
        .contains(&project.get_name()));
}
