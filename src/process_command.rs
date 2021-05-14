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
            if !xs.is_empty() {
                let _out = match xs.split_at(1) {
                    (&[cmd], args) => args
                        .iter()
                        .fold(&mut Command::new(cmd), |s, i| s.arg(i))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .output()?,
                    _ => unreachable!(),
                };
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod process_command_tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), FtwError> {
        let commands = vec![vec!["ls", "-al"], vec!["cat", "Cargo.toml"]];
        let result = ProcessCommand { commands }.process();
        assert!(result.is_ok());
        assert_eq!(result?, ());
        Ok(())
    }

    #[test]
    fn test_process2() -> Result<(), FtwError> {
        let commands = vec![vec!["ls"], vec!["cat", "Cargo.toml"]];
        let result = ProcessCommand { commands }.process();
        assert!(result.is_ok());
        assert_eq!(result?, ());
        Ok(())
    }

    #[test]
    fn test_process_error() {
        let commands = vec![vec!["some-invalid.commad"], vec!["another.invalid-command"]];
        let result = ProcessCommand { commands }.process();
        assert!(result.is_err());
        match result.unwrap_err() {
            FtwError::Error(_) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_process_error2() {
        let commands = vec![vec!["exit", "1"], vec!["another.invalid-command"]];
        let result = ProcessCommand { commands }.process();
        assert!(result.is_err());
        match result.unwrap_err() {
            FtwError::Error(_) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_process_error3() {
        let commands = vec![vec!["", "1"], vec!["another.invalid-command"]];
        let result = ProcessCommand { commands }.process();
        assert!(result.is_err());
        match result.unwrap_err() {
            FtwError::Error(_) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_process_empty_command() -> Result<(), FtwError> {
        let commands = vec![vec![], vec![]];
        let result = ProcessCommand { commands }.process();
        assert!(result.is_ok());
        assert_eq!(result?, ());
        Ok(())
    }
}
