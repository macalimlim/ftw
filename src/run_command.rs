use crate::ftw_error::FtwError;
use crate::traits::Runner;
use std::process::{Command, Stdio};

impl Runner for Command {
    fn run(&mut self, current_dir: &str) -> Result<(), FtwError> {
        self.current_dir(current_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;
        Ok(())
    }
}

#[cfg(test)]
mod run_command_tests {
    use super::*;
    use command_macros::cmd;

    #[test]
    fn test_run() -> Result<(), FtwError> {
        let result = cmd!(ls("-al")).run(".");
        assert!(result.is_ok());
        assert_eq!(result?, ());
        let result = cmd!(cat("Cargo.toml")).run(".");
        assert!(result.is_ok());
        assert_eq!(result?, ());
        Ok(())
    }

    #[test]
    fn test_run_error() {
        let result = cmd!(gogogo).run(".");
        assert!(result.is_err());
        match result.unwrap_err() {
            FtwError::Error(_) => assert!(true),
            _ => unreachable!(),
        }
    }
}
