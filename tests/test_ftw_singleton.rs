mod common;

use assert_cmd::prelude::*;
use common::{ftw, generate_random_name, Project};
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_singleton() {
    let name = generate_random_name();
    ftw()
        .arg("new")
        .arg(&name)
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    let project = Project::new(&name);
    ftw()
        .arg("singleton")
        .arg("MyPlayer")
        .current_dir(&name)
        .assert()
        .success();
    assert!(project.exists("rust/src/my_player.rs"));
    assert!(project.exists("godot/native/MyPlayer.gdns"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/my_player.rs")
        .contains("pub struct MyPlayer"));
    assert!(project
        .read("rust/src/my_player.rs")
        .contains("#[inherit(Node)]"));
    assert!(project
        .read("godot/native/MyPlayer.gdns")
        .contains("resource_name = \"MyPlayer\""));
    assert!(project
        .read("godot/native/MyPlayer.gdns")
        .contains("class_name = \"MyPlayer\""));
    assert!(project.read("rust/src/lib.rs").contains("mod my_player;"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("handle.add_class::<my_player::MyPlayer>();"));
}

#[test]
fn test_ftw_singleton_no_class_name() {
    let name = generate_random_name();
    ftw()
        .arg("new")
        .arg(&name)
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("singleton")
        .arg("")
        .current_dir(&name)
        .assert()
        .failure()
        .stderr(predicates::str::contains("error").from_utf8());
}
