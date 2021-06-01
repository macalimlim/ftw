use assert_cmd::prelude::*;
use nanoid::nanoid;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::str;

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
}

impl Project {
    pub fn new(root: &String) -> Self {
        Project {
            root: PathBuf::from(root),
        }
    }

    pub fn read(&self, path: &str) -> String {
        let mut ret = String::new();
        File::open(self.root.join(path))
            .expect(&format!("couldn't open file {:?}", self.root.join(path)))
            .read_to_string(&mut ret)
            .expect(&format!("couldn't read file {:?}", self.root.join(path)));
        return ret;
    }

    pub fn exists(&self, path: &str) -> bool {
        self.root.join(path).exists()
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
        drop(fs::remove_dir(&self.root));
    }
}

pub fn generate_random_name() -> String {
    let name = nanoid!();
    let mut name = name.to_lowercase().replace("_", "-").replace("-", "");
    name.insert_str(0, "game");
    name
}

pub fn ftw() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}
