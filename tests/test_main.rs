use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::env;
use std::fs::{remove_dir_all, File};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

const GAME: &str = "my-awesome-game";

fn ftw() -> std::process::Command {
    Command::cargo_bin("ftw").unwrap()
}

fn setup() {}

fn teardown() {
    remove_dir_all(GAME);
}

fn test_generated_project() {
    assert!(Path::new(GAME).exists());
    let gitignore_file_path = format!("{}/.gitignore", GAME);
    let mut gitignore_file = File::open(gitignore_file_path).unwrap();
    let mut gitignore_file_contents = String::new();
    gitignore_file
        .read_to_string(&mut gitignore_file_contents)
        .unwrap();
    assert!(gitignore_file_contents.contains("export_presets.cfg"));
}

#[test]
fn test_ftw_new() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let current_path = env::current_dir()?;
    ftw()
        .arg("new")
        .arg(GAME)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Done! New project created {}/{}",
            current_path.display(),
            GAME
        )));
    test_generated_project();
    teardown();
    Ok(())
}

#[test]
fn test_ftw_new_failure() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    ftw()
        .arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "The following required arguments were not provided:\n    <project_name>\n",
        ));
    teardown();
    Ok(())
}

#[test]
fn test_ftw_new_with_default_template() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let current_path = env::current_dir()?;
    ftw()
        .arg("new")
        .arg(GAME)
        .arg("default")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Done! New project created {}/{}",
            current_path.display(),
            GAME
        )));
    test_generated_project();
    teardown();
    Ok(())
}

#[test]
fn test_ftw_new_with_custom_template() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let current_path = env::current_dir()?;
    ftw()
        .arg("new")
        .arg(GAME)
        .arg("../ftw")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Done! New project created {}/{}",
            current_path.display(),
            GAME
        )));
    test_generated_project();
    teardown();
    Ok(())
}
