use crate::ftw_configuration::FtwConfiguration;
use crate::ftw_target::FtwTarget;
use std::env;

pub fn get_current_platform() -> String {
    format!("{}-{}", env::consts::OS, env::consts::ARCH)
}

pub fn get_class_name_and_directories(class_name: &str) -> (String, Vec<String>) {
    let xs: Vec<&str> = class_name.split('/').collect();
    match xs.split_last() {
        Some((class_name, directories)) => (
            class_name.to_string(),
            directories.to_vec().iter().map(|d| d.to_string()).collect(),
        ),
        _ => unreachable!(),
    }
}

pub fn get_godot_exe_for_exporting() -> String {
    let current_platform = get_current_platform()
        .parse()
        .unwrap_or(FtwTarget::WindowsX86_64Msvc);
    let ftw_cfg = FtwConfiguration::new();
    match current_platform {
        FtwTarget::LinuxX86 | FtwTarget::LinuxX86_64 => ftw_cfg.godot_headless_executable,
        _ => ftw_cfg.godot_executable,
    }
}
