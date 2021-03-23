use ini::{Ini, Properties};

pub struct FtwConfiguration {
    pub godot_executable: String,
    pub godot_headless_executable: String,
    pub godot_server_executable: String,
}

impl FtwConfiguration {
    pub fn new() -> Self {
        let ini = Ini::load_from_file(".ftw").unwrap_or_default();
        let default_properties = Properties::new();
        let ftw_section = ini.section(Some("ftw")).unwrap_or(&default_properties);
        let exekey_default_pairs = vec![
            ("godot-exe", "godot"),
            ("godot-headless-exe", "godot-headless"),
            ("godot-server-exe", "godot-server"),
        ];
        let exes: Vec<String> = exekey_default_pairs
            .iter()
            .map(|(exe, def)| ftw_section.get(exe).unwrap_or(def).replace("\\", "/"))
            .collect();
        match exes.as_slice() {
            [ge, ghe, gse] => FtwConfiguration {
                godot_executable: ge.to_string(),
                godot_headless_executable: ghe.to_string(),
                godot_server_executable: gse.to_string(),
            },
            _ => unreachable!(),
        }
    }
}

impl Default for FtwConfiguration {
    fn default() -> Self {
        FtwConfiguration {
            godot_executable: "godot".to_string(),
            godot_headless_executable: "godot-headless".to_string(),
            godot_server_executable: "godot-server".to_string(),
        }
    }
}
