use crate::ftw_build_type::FtwBuildType;
use crate::ftw_compiler::FtwCompiler;
use crate::ftw_target::FtwTarget;
use ini::{Ini, Properties};

pub const GODOT_EXE: &str = "godot";
pub const GODOT_HEADLESS_EXE: &str = "godot-headless";
pub const GODOT_SERVER_EXE: &str = "godot-server";

#[derive(Debug, Eq, PartialEq)]
pub struct FtwConfiguration {
    pub godot_executable: String,
    pub godot_headless_executable: String,
    pub godot_server_executable: String,
    pub enable_cross_compilation: bool,
}

impl FtwConfiguration {
    #[must_use]
    pub fn new() -> Self {
        let ini = Ini::load_from_file(".ftw").unwrap_or_default();
        let default_properties = Properties::new();
        let ftw_section = ini.section(Some("ftw")).unwrap_or(&default_properties);
        let exe_key_default_pairs = vec![
            ("godot-exe", GODOT_EXE),
            ("godot-headless-exe", GODOT_HEADLESS_EXE),
            ("godot-server-exe", GODOT_SERVER_EXE),
            ("enable-cross-compilation", "false"),
        ];
        let keys: Vec<String> = exe_key_default_pairs
            .iter()
            .map(|(key, def)| ftw_section.get(key).unwrap_or(def).replace('\\', "/"))
            .collect();
        match keys.as_slice() {
            [godot_exe, godot_headless_exe, godot_server_exe, enable_cross_compilation] => {
                FtwConfiguration {
                    godot_executable: godot_exe.to_string(),
                    godot_headless_executable: godot_headless_exe.to_string(),
                    godot_server_executable: godot_server_exe.to_string(),
                    enable_cross_compilation: enable_cross_compilation == "true",
                }
            }
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub fn get_compiler(&self, target: FtwTarget, build_type: FtwBuildType) -> FtwCompiler {
        if self.enable_cross_compilation {
            FtwCompiler::Cross { target, build_type }
        } else {
            FtwCompiler::Local { target, build_type }
        }
    }
}

impl Default for FtwConfiguration {
    fn default() -> Self {
        FtwConfiguration {
            godot_executable: GODOT_EXE.to_string(),
            godot_headless_executable: GODOT_HEADLESS_EXE.to_string(),
            godot_server_executable: GODOT_SERVER_EXE.to_string(),
            enable_cross_compilation: false,
        }
    }
}

#[cfg(test)]
mod ftw_configuration_tests {
    use super::*;

    #[test]
    fn test_default() {
        let cfg = FtwConfiguration {
            godot_executable: GODOT_EXE.to_string(),
            godot_headless_executable: GODOT_HEADLESS_EXE.to_string(),
            godot_server_executable: GODOT_SERVER_EXE.to_string(),
            enable_cross_compilation: false,
        };
        assert_eq!(FtwConfiguration::default(), cfg);
    }
}
