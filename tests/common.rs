use assert_cmd::prelude::*;
use std::process::Command;

pub fn ftw() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}
