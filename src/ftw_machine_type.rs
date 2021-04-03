use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum FtwMachineType {
    Desktop,
    Server,
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

#[cfg(test)]
mod ftw_machine_type_tests {
    use super::*;
    use proptest::prelude::{prop_assert, prop_assert_eq, prop_assume, proptest};

    #[test]
    fn test_from_str() -> Result<(), ()> {
        assert_eq!(FtwMachineType::Desktop, "desktop".parse()?);
        assert_eq!(FtwMachineType::Server, "server".parse()?);
        Ok(())
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
