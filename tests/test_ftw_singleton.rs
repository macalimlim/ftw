mod common;

use assert_cmd::prelude::*;
use common::ftw;
use ftw::test_util::Project;
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_singleton() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("singleton")
        .arg("MyPlayer")
        .current_dir(&project.get_name())
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
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("singleton")
        .current_dir(&project.get_name())
        .assert()
        .failure()
        .stderr(predicates::str::contains("error").from_utf8());
    drop(project)
}

#[test]
fn test_ftw_singleton_with_subs() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("singleton")
        .arg("foo/bar/baz/MyPlayer")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/foo/bar/baz/my_player.rs"));
    assert!(project.exists("rust/src/foo/bar/baz/mod.rs"));
    assert!(project.exists("rust/src/foo/bar/mod.rs"));
    assert!(project.exists("rust/src/foo/mod.rs"));
    assert!(project.exists("godot/native/foo/bar/baz/MyPlayer.gdns"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/foo/bar/baz/my_player.rs")
        .contains("pub struct MyPlayer"));
    assert!(project
        .read("rust/src/foo/bar/baz/my_player.rs")
        .contains("#[inherit(Node)]"));
    assert!(project
        .read("godot/native/foo/bar/baz/MyPlayer.gdns")
        .contains("resource_name = \"MyPlayer\""));
    assert!(project
        .read("godot/native/foo/bar/baz/MyPlayer.gdns")
        .contains("class_name = \"MyPlayer\""));
    assert!(project.read("rust/src/lib.rs").contains("mod foo;"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("handle.add_class::<foo::bar::baz::my_player::MyPlayer>();"));
    assert!(project
        .read("rust/src/foo/bar/baz/mod.rs")
        .contains("pub mod my_player;"));
    assert!(project
        .read("rust/src/foo/bar/mod.rs")
        .contains("pub mod baz;"));
    assert!(project.read("rust/src/foo/mod.rs").contains("pub mod bar;"));
    //
    ftw()
        .arg("singleton")
        .arg("foo/bar/FooBar")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/foo/bar/foo_bar.rs"));
    assert!(project.exists("rust/src/foo/bar/mod.rs"));
    assert!(project.exists("rust/src/foo/mod.rs"));
    assert!(project.exists("godot/native/foo/bar/FooBar.gdns"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/foo/bar/foo_bar.rs")
        .contains("pub struct FooBar"));
    assert!(project
        .read("rust/src/foo/bar/foo_bar.rs")
        .contains("#[inherit(Node)]"));
    assert!(project
        .read("godot/native/foo/bar/FooBar.gdns")
        .contains("resource_name = \"FooBar\""));
    assert!(project
        .read("godot/native/foo/bar/FooBar.gdns")
        .contains("class_name = \"FooBar\""));
    assert!(project.read("rust/src/lib.rs").contains("mod foo;"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("handle.add_class::<foo::bar::foo_bar::FooBar>();"));
    assert!(project
        .read("rust/src/foo/bar/mod.rs")
        .contains("pub mod foo_bar;"));
    assert!(project
        .read("rust/src/foo/bar/mod.rs")
        .contains("pub mod baz;"));
    assert!(project.read("rust/src/foo/mod.rs").contains("pub mod bar;"));
    //
    ftw()
        .arg("singleton")
        .arg("foo/bar/baz/woot/Blah")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/foo/bar/baz/woot/blah.rs"));
    assert!(project.exists("rust/src/foo/bar/baz/woot/mod.rs"));
    assert!(project.exists("rust/src/foo/bar/baz/mod.rs"));
    assert!(project.exists("rust/src/foo/bar/mod.rs"));
    assert!(project.exists("rust/src/foo/mod.rs"));
    assert!(project.exists("godot/native/foo/bar/baz/woot/Blah.gdns"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/foo/bar/baz/woot/blah.rs")
        .contains("pub struct Blah"));
    assert!(project
        .read("rust/src/foo/bar/baz/woot/blah.rs")
        .contains("#[inherit(Node)]"));
    assert!(project
        .read("godot/native/foo/bar/baz/woot/Blah.gdns")
        .contains("resource_name = \"Blah\""));
    assert!(project
        .read("godot/native/foo/bar/baz/woot/Blah.gdns")
        .contains("class_name = \"Blah\""));
    assert!(project.read("rust/src/lib.rs").contains("mod foo;"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("handle.add_class::<foo::bar::baz::woot::blah::Blah>();"));
    assert!(project
        .read("rust/src/foo/bar/baz/woot/mod.rs")
        .contains("pub mod blah;"));
    assert!(project
        .read("rust/src/foo/bar/baz/mod.rs")
        .contains("pub mod woot;"));
    assert!(project
        .read("rust/src/foo/bar/baz/mod.rs")
        .contains("pub mod my_player;"));
    assert!(project.read("rust/src/foo/mod.rs").contains("pub mod bar;"));
}
