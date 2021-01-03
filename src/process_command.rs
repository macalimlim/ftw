use crate::ftw_error::FtwError;
use crate::traits::Processor;
use crate::type_alias::Commands;
use std::io::{self, Write};
use std::process::Command;

pub struct ProcessCommand<'a> {
    pub commands: Commands<'a>,
}

impl Processor for ProcessCommand<'_> {
    fn process(&self) -> Result<(), FtwError> {
        for xs in &self.commands {
            let out = match xs.split_at(1) {
                (&[cmd], args) => args
                    .iter()
                    .fold(&mut Command::new(cmd), |s, i| s.arg(i))
                    .output()?,
                _ => unreachable!(),
            };
            if out.status.success() {
                io::stdout().write_all(&out.stdout)?;
            } else {
                io::stderr().write_all(&out.stderr)?;
            }
        }
        Ok(())
    }
}
