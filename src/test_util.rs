#![allow(dead_code)]
use nanoid::nanoid;
use std::fs::{remove_dir, remove_dir_all, File};
use std::io::Read;
use std::path::PathBuf;
use std::str;

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
}

impl Project {
    #[must_use]
    pub fn new() -> Self {
        let name = nanoid!();
        let mut name = name.to_lowercase().replace("_", "-").replace("-", "");
        name.insert_str(0, "game");
        Project {
            root: PathBuf::from(name),
        }
    }

    /// # Panics
    ///
    /// Will panic if it couldn't open or read the file
    #[must_use]
    pub fn read(&self, path: &str) -> String {
        let mut ret = String::new();
        File::open(self.root.join(path))
            .unwrap_or_else(|_| panic!("couldn't open file {:?}", self.root.join(path)))
            .read_to_string(&mut ret)
            .unwrap_or_else(|_| panic!("couldn't read file {:?}", self.root.join(path)));
        ret
    }

    #[must_use]
    pub fn exists(&self, path: &str) -> bool {
        self.root.join(path).exists()
    }

    #[must_use]
    pub fn get_name(&self) -> String {
        let pathbuf = self.root.clone();
        pathbuf
            .into_os_string()
            .into_string()
            .unwrap_or_else(|_| "gameabc123xyz456".to_string())
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        drop(remove_dir_all(&self.root));
        drop(remove_dir(&self.root));
    }
}
