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
