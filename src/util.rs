use std::env;

pub fn get_current_platform() -> String {
    format!("{}-{}", env::consts::OS, env::consts::ARCH)
}
