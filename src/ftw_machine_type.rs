use std::str::FromStr;

#[derive(PartialEq)]
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
