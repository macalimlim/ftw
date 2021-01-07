mod common;

use assert_cmd::prelude::*;
use common::{ftw, generate_random_name, Project};
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_new() {
    let name = generate_random_name();
    let project = Project::new(&name);
    ftw()
        .arg("new")
        .arg(&name)
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
    assert!(project.read(".gitignore").contains("export_presets.cfg"));
    assert!(project.read("rust/Cargo.toml").contains(&name));
}

#[test]
fn test_ftw_new_no_template() {
    let name = generate_random_name();
    let project = Project::new(&name);
    ftw()
        .arg("new")
        .arg(&name)
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
    assert!(project.read(".gitignore").contains("export_presets.cfg"));
    assert!(project.read("rust/Cargo.toml").contains(&name));
}
