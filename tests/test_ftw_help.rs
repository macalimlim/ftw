mod common;

use assert_cmd::prelude::*;
use clap::{crate_authors, crate_description, crate_name, crate_version};
use common::ftw;
use predicates;
use predicates::prelude::*;

#[test]
fn test_ftw_help() {
    let crate_authors = crate_authors!();
    let crate_description = crate_description!();
    let crate_name = crate_name!();
    let crate_version = crate_version!();
    let name_version = format!("{crate_name} {crate_version}");
    let usage = format!("Usage: {crate_name} [COMMAND]");
    ftw()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains(name_version).from_utf8())
        .stdout(predicates::str::contains(crate_description).from_utf8())
        .stdout(predicates::str::contains(crate_authors).from_utf8())
        .stdout(predicates::str::contains(usage).from_utf8());
}
