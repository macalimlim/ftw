use crate::traits::Processor;
use crate::type_alias::Commands;
use std::io::{self, Write};
use std::process::Command;

pub struct ProcessCommand<'a> {
    pub commands: Commands<'a>,
}

impl Processor for ProcessCommand<'_> {
    fn process(&self) {
        for xs in &self.commands {
            let out = match xs.split_at(1) {
                (&[cmd], args) => args
                    .iter()
                    .fold(&mut Command::new(cmd), |s, i| s.arg(i))
                    .output()
                    .expect("failed to execute process"),
                _ => panic!("this should not happen"),
            };
            if out.status.success() {
                io::stdout().write_all(&out.stdout).unwrap();
            } else {
                io::stderr().write_all(&out.stderr).unwrap();
            }
        }
    }
}
