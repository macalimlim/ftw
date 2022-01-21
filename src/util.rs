use crate::ftw_configuration::FtwConfiguration;
use crate::ftw_machine_type::FtwMachineType;
use crate::ftw_target::FtwTarget;
use std::env;

#[must_use]
pub fn get_current_platform() -> String {
    format!("{}-{}", env::consts::OS, env::consts::ARCH)
}

#[must_use]
pub fn get_class_name_and_directories(class_name: &str) -> (String, Vec<String>) {
    let xs: Vec<&str> = class_name.split('/').collect();
    match xs.split_last() {
        Some((class_name, directories)) => (
            (*class_name).to_string(),
            directories
                .to_vec()
                .iter()
                .map(|d| (*d).to_string())
                .collect(),
        ),
        _ => unreachable!(),
    }
}

#[must_use]
pub fn get_godot_exe_for_exporting(current_platform: FtwTarget) -> String {
    let ftw_cfg = FtwConfiguration::new();
    match current_platform {
        FtwTarget::LinuxX86 | FtwTarget::LinuxX86_64 => ftw_cfg.godot_headless_executable,
        _ => ftw_cfg.godot_executable,
    }
}

#[must_use]
pub fn get_godot_exe_for_running(machine_type: &FtwMachineType) -> String {
    let ftw_cfg = FtwConfiguration::new();
    match machine_type {
        FtwMachineType::Desktop => ftw_cfg.godot_executable,
        FtwMachineType::Server => ftw_cfg.godot_server_executable,
    }
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_get_godot_exe_for_exporting() {
        let linux_desktop_platforms = vec![FtwTarget::LinuxX86, FtwTarget::LinuxX86_64];
        for p in linux_desktop_platforms {
            let godot_exe = get_godot_exe_for_exporting(p);
            assert_eq!("godot-headless".to_string(), godot_exe);
        }
        let other_desktop_platforms = vec![
            FtwTarget::MacOsX86_64,
            FtwTarget::WindowsX86Gnu,
            FtwTarget::WindowsX86Msvc,
            FtwTarget::WindowsX86_64Gnu,
            FtwTarget::WindowsX86_64Msvc,
        ];
        for p in other_desktop_platforms {
            let godot_exe = get_godot_exe_for_exporting(p);
            assert_eq!("godot".to_string(), godot_exe);
        }
    }

    #[test]
    fn test_get_godot_exe_for_running() {
        let machine_type = FtwMachineType::Desktop;
        let godot_exe = get_godot_exe_for_running(&machine_type);
        assert_eq!("godot".to_string(), godot_exe);
        let machine_type = FtwMachineType::Server;
        let godot_exe = get_godot_exe_for_running(&machine_type);
        assert_eq!("godot-server".to_string(), godot_exe);
    }

    #[test]
    fn test_get_class_name_and_directories() {
        let class_name = "IronMan";
        let v = get_class_name_and_directories(class_name);
        assert_eq!(("IronMan".to_string(), vec![]), v);
    }

    #[test]
    fn test_get_class_name_and_directories_with_slashes() {
        let class_name = "marvel/avengers/IronMan";
        let v = get_class_name_and_directories(class_name);
        assert_eq!(
            (
                "IronMan".to_string(),
                vec!["marvel".to_string(), "avengers".to_string()]
            ),
            v
        );
    }
}
