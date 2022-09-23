use crate::traits::ToCliArg;
use crate::type_alias::CliArg;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum FtwMachineType {
    Desktop,
    Server,
}

impl FtwMachineType {
    #[must_use]
    pub fn is_desktop(&self) -> bool {
        self == &FtwMachineType::Desktop
    }

    #[must_use]
    pub fn is_server(&self) -> bool {
        !self.is_desktop()
    }
}

impl FromStr for FtwMachineType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "server" => Ok(FtwMachineType::Server),
            _ => Ok(FtwMachineType::Desktop),
        }
    }
}

impl Display for FtwMachineType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            FtwMachineType::Desktop => "desktop",
            FtwMachineType::Server => "server",
        };
        write!(f, "{}", message)
    }
}

impl ToCliArg for FtwMachineType {
    fn to_cli_arg(&self) -> CliArg {
        match self {
            FtwMachineType::Desktop => "-d",
            FtwMachineType::Server => "",
        }
        .to_string()
    }
}

impl Default for FtwMachineType {
    fn default() -> Self {
        FtwMachineType::Desktop
    }
}

#[cfg(test)]
mod ftw_machine_type_tests {
    use super::*;
    use proptest::prelude::{prop_assert, prop_assert_eq, prop_assume, proptest};

    #[test]
    fn test_is_desktop() {
        assert!(FtwMachineType::Desktop.is_desktop());
        assert!(!FtwMachineType::Server.is_desktop());
    }

    #[test]
    fn test_is_server() {
        assert!(!FtwMachineType::Desktop.is_server());
        assert!(FtwMachineType::Server.is_server());
    }

    #[test]
    fn test_from_str() -> Result<(), ()> {
        assert_eq!(FtwMachineType::Desktop, "desktop".parse()?);
        assert_eq!(FtwMachineType::Server, "server".parse()?);
        Ok(())
    }

    #[test]
    fn test_fmt() {
        assert_eq!("desktop", format!("{}", FtwMachineType::Desktop));
        assert_eq!("server", format!("{}", FtwMachineType::Server));
    }

    #[test]
    fn test_to_cli_arg() {
        assert_eq!("-d", FtwMachineType::Desktop.to_cli_arg());
        assert_eq!("", FtwMachineType::Server.to_cli_arg());
    }

    #[test]
    fn test_default() {
        assert_eq!(FtwMachineType::default(), FtwMachineType::Desktop);
    }

    proptest! {
        #[test]
        fn test_from_str_invalid_input(machine_type_input in "\\PC*") {
            prop_assume!(machine_type_input != "desktop");
            prop_assume!(machine_type_input != "server");
            prop_assert!(machine_type_input.parse::<FtwMachineType>().is_ok());
            prop_assert_eq!(FtwMachineType::Desktop, machine_type_input.parse::<FtwMachineType>().unwrap());
        }
    }
}
