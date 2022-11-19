mod common;

use assert_cmd::prelude::*;
use common::ftw;
use ftw::test_util::Project;
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_class() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("class")
        .arg("MyPlayer")
        .arg("Area2D")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/my_player.rs"));
    assert!(project.exists("godot/native/MyPlayer.gdns"));
    assert!(project.exists("godot/scenes/MyPlayer.tscn"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/my_player.rs")
        .contains("pub struct MyPlayer"));
    assert!(project
        .read("rust/src/my_player.rs")
        .contains("#[inherit(Area2D)]"));
    assert!(project
        .read("godot/native/MyPlayer.gdns")
        .contains("resource_name = \"MyPlayer\""));
    assert!(project
        .read("godot/native/MyPlayer.gdns")
        .contains("class_name = \"MyPlayer\""));
    assert!(project
        .read("godot/scenes/MyPlayer.tscn")
        .contains("[ext_resource path=\"res://native/MyPlayer.gdns\" type=\"Script\" id=1]"));
    assert!(project
        .read("godot/scenes/MyPlayer.tscn")
        .contains("[node name=\"MyPlayer\" type=\"Area2D\"]"));
    assert!(project.read("rust/src/lib.rs").contains("mod my_player;"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("handle.add_class::<my_player::MyPlayer>();"));
}

#[test]
fn test_ftw_tool_class() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("class")
        .arg("MyButtonTool")
        .arg("Button")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/my_button_tool.rs"));
    assert!(project.exists("godot/native/MyButtonTool.gdns"));
    assert!(project.exists("godot/scenes/MyButtonTool.tscn"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/my_button_tool.rs")
        .contains("pub struct MyButtonTool"));
    assert!(project
        .read("rust/src/my_button_tool.rs")
        .contains("#[inherit(Button)]"));
    assert!(project
        .read("godot/native/MyButtonTool.gdns")
        .contains("resource_name = \"MyButtonTool\""));
    assert!(project
        .read("godot/native/MyButtonTool.gdns")
        .contains("class_name = \"MyButtonTool\""));
    assert!(project
        .read("godot/scenes/MyButtonTool.tscn")
        .contains("[ext_resource path=\"res://native/MyButtonTool.gdns\" type=\"Script\" id=1]"));
    assert!(project
        .read("godot/scenes/MyButtonTool.tscn")
        .contains("[node name=\"MyButtonTool\" type=\"Button\"]"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("mod my_button_tool;"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("handle.add_tool_class::<my_button_tool::MyButtonTool>();"));
}

#[test]
fn test_ftw_class_no_node_type() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("class")
        .arg("MyPlayer")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/my_player.rs"));
    assert!(project.exists("godot/native/MyPlayer.gdns"));
    assert!(project.exists("godot/scenes/MyPlayer.tscn"));
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
    assert!(project
        .read("godot/scenes/MyPlayer.tscn")
        .contains("[ext_resource path=\"res://native/MyPlayer.gdns\" type=\"Script\" id=1]"));
    assert!(project
        .read("godot/scenes/MyPlayer.tscn")
        .contains("[node name=\"MyPlayer\" type=\"Node\"]"));
    assert!(project.read("rust/src/lib.rs").contains("mod my_player;"));
    assert!(project
        .read("rust/src/lib.rs")
        .contains("handle.add_class::<my_player::MyPlayer>();"));
}

#[test]
fn test_ftw_class_no_class_name() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("class")
        .current_dir(&project.get_name())
        .assert()
        .failure()
        .stderr(predicates::str::contains("error").from_utf8());
    drop(project)
}

#[test]
fn test_ftw_class_with_subs() {
    let project = Project::new();
    ftw()
        .arg("new")
        .arg(&project.get_name())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());
    ftw()
        .arg("class")
        .arg("foo/bar/baz/MyPlayer")
        .arg("Area2D")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/foo/bar/baz/my_player.rs"));
    assert!(project.exists("rust/src/foo/bar/baz/mod.rs"));
    assert!(project.exists("rust/src/foo/bar/mod.rs"));
    assert!(project.exists("rust/src/foo/mod.rs"));
    assert!(project.exists("godot/native/foo/bar/baz/MyPlayer.gdns"));
    assert!(project.exists("godot/scenes/foo/bar/baz/MyPlayer.tscn"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/foo/bar/baz/my_player.rs")
        .contains("pub struct MyPlayer"));
    assert!(project
        .read("rust/src/foo/bar/baz/my_player.rs")
        .contains("#[inherit(Area2D)]"));
    assert!(project
        .read("godot/native/foo/bar/baz/MyPlayer.gdns")
        .contains("resource_name = \"MyPlayer\""));
    assert!(project
        .read("godot/native/foo/bar/baz/MyPlayer.gdns")
        .contains("class_name = \"MyPlayer\""));
    assert!(project
        .read("godot/scenes/foo/bar/baz/MyPlayer.tscn")
        .contains(
            "[ext_resource path=\"res://native/foo/bar/baz/MyPlayer.gdns\" type=\"Script\" id=1]"
        ));
    assert!(project
        .read("godot/scenes/foo/bar/baz/MyPlayer.tscn")
        .contains("[node name=\"MyPlayer\" type=\"Area2D\"]"));
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
        .arg("class")
        .arg("foo/bar/FooBar")
        .arg("Area2D")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/foo/bar/foo_bar.rs"));
    assert!(project.exists("rust/src/foo/bar/mod.rs"));
    assert!(project.exists("rust/src/foo/mod.rs"));
    assert!(project.exists("godot/native/foo/bar/FooBar.gdns"));
    assert!(project.exists("godot/scenes/foo/bar/FooBar.tscn"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/foo/bar/foo_bar.rs")
        .contains("pub struct FooBar"));
    assert!(project
        .read("rust/src/foo/bar/foo_bar.rs")
        .contains("#[inherit(Area2D)]"));
    assert!(project
        .read("godot/native/foo/bar/FooBar.gdns")
        .contains("resource_name = \"FooBar\""));
    assert!(project
        .read("godot/native/foo/bar/FooBar.gdns")
        .contains("class_name = \"FooBar\""));
    assert!(project
        .read("godot/scenes/foo/bar/FooBar.tscn")
        .contains("[ext_resource path=\"res://native/foo/bar/FooBar.gdns\" type=\"Script\" id=1]"));
    assert!(project
        .read("godot/scenes/foo/bar/FooBar.tscn")
        .contains("[node name=\"FooBar\" type=\"Area2D\"]"));
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
        .arg("class")
        .arg("foo/bar/baz/woot/Blah")
        .arg("Area2D")
        .current_dir(&project.get_name())
        .assert()
        .success();
    assert!(project.exists("rust/src/foo/bar/baz/woot/blah.rs"));
    assert!(project.exists("rust/src/foo/bar/baz/woot/mod.rs"));
    assert!(project.exists("rust/src/foo/bar/baz/mod.rs"));
    assert!(project.exists("rust/src/foo/bar/mod.rs"));
    assert!(project.exists("rust/src/foo/mod.rs"));
    assert!(project.exists("godot/native/foo/bar/baz/woot/Blah.gdns"));
    assert!(project.exists("godot/scenes/foo/bar/baz/woot/Blah.tscn"));
    assert!(project.exists("rust/src/lib.rs"));
    assert!(project
        .read("rust/src/foo/bar/baz/woot/blah.rs")
        .contains("pub struct Blah"));
    assert!(project
        .read("rust/src/foo/bar/baz/woot/blah.rs")
        .contains("#[inherit(Area2D)]"));
    assert!(project
        .read("godot/native/foo/bar/baz/woot/Blah.gdns")
        .contains("resource_name = \"Blah\""));
    assert!(project
        .read("godot/native/foo/bar/baz/woot/Blah.gdns")
        .contains("class_name = \"Blah\""));
    assert!(project
        .read("godot/scenes/foo/bar/baz/woot/Blah.tscn")
        .contains(
            "[ext_resource path=\"res://native/foo/bar/baz/woot/Blah.gdns\" type=\"Script\" id=1]"
        ));
    assert!(project
        .read("godot/scenes/foo/bar/baz/woot/Blah.tscn")
        .contains("[node name=\"Blah\" type=\"Area2D\"]"));
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
