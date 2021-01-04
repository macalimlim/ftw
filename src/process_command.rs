use crate::ftw_error::FtwError;
use crate::traits::Processor;
use crate::type_alias::Commands;
use std::process::{Command, Stdio};

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
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()?,
                _ => return Err(FtwError::InvalidCommandError),
            };
            if !out.status.success() {
                return Err(FtwError::ProcessCommandError);
            }
        }
        Ok(())
    }
}
