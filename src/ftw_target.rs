use crate::ftw_error::FtwError;
use crate::traits::{ToAppExt, ToCliArg, ToExportName, ToLibExt, ToLibPrefix};
use crate::type_alias::{AppExt, CliArg, ExportName, LibExt, LibPrefix};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum FtwTarget {
    AndroidLinuxAarch64,
    AndroidLinuxArmV7,
    AndroidLinuxX86,
    AndroidLinuxX86_64,
    IosAarch64,
    LinuxX86,
    LinuxX86_64,
    MacOsX86_64,
    WindowsX86Gnu,
    WindowsX86Msvc,
    WindowsX86_64Gnu,
    WindowsX86_64Msvc,
}

impl ToCliArg for FtwTarget {
    fn to_cli_arg(&self) -> CliArg {
        match self {
            FtwTarget::AndroidLinuxAarch64 => "aarch64-linux-android",
            FtwTarget::AndroidLinuxArmV7 => "armv7-linux-androideabi",
            FtwTarget::AndroidLinuxX86 => "i686-linux-android",
            FtwTarget::AndroidLinuxX86_64 => "x86_64-linux-android",
            FtwTarget::IosAarch64 => "aarch64-apple-ios",
            FtwTarget::LinuxX86 => "i686-unknown-linux-gnu",
            FtwTarget::LinuxX86_64 => "x86_64-unknown-linux-gnu",
            FtwTarget::MacOsX86_64 => "x86_64-apple-darwin",
            FtwTarget::WindowsX86Gnu => "i686-pc-windows-gnu",
            FtwTarget::WindowsX86Msvc => "i686-pc-windows-msvc",
            FtwTarget::WindowsX86_64Gnu => "x86_64-pc-windows-gnu",
            FtwTarget::WindowsX86_64Msvc => "x86_64-pc-windows-msvc",
        }
        .to_string()
    }
}

impl ToExportName for FtwTarget {
    fn to_export_name(&self) -> ExportName {
        match self {
            FtwTarget::AndroidLinuxAarch64
            | FtwTarget::AndroidLinuxArmV7
            | FtwTarget::AndroidLinuxX86
            | FtwTarget::AndroidLinuxX86_64 => "Android",
            FtwTarget::IosAarch64 => "iOS",
            FtwTarget::LinuxX86 | FtwTarget::LinuxX86_64 => "Linux/X11",
            FtwTarget::MacOsX86_64 => "Mac OSX",
            FtwTarget::WindowsX86Gnu
            | FtwTarget::WindowsX86Msvc
            | FtwTarget::WindowsX86_64Gnu
            | FtwTarget::WindowsX86_64Msvc => "Windows",
        }
        .to_string()
    }
}

impl ToAppExt for FtwTarget {
    fn to_app_ext(&self) -> AppExt {
        match self {
            FtwTarget::AndroidLinuxAarch64
            | FtwTarget::AndroidLinuxArmV7
            | FtwTarget::AndroidLinuxX86
            | FtwTarget::AndroidLinuxX86_64 => ".apk",
            FtwTarget::IosAarch64 => ".ipa",
            FtwTarget::LinuxX86 | FtwTarget::LinuxX86_64 | FtwTarget::MacOsX86_64 => "",
            FtwTarget::WindowsX86Gnu
            | FtwTarget::WindowsX86Msvc
            | FtwTarget::WindowsX86_64Gnu
            | FtwTarget::WindowsX86_64Msvc => ".exe",
        }
        .to_string()
    }
}

impl ToLibExt for FtwTarget {
    fn to_lib_ext(&self) -> LibExt {
        match self {
            FtwTarget::AndroidLinuxAarch64
            | FtwTarget::AndroidLinuxArmV7
            | FtwTarget::AndroidLinuxX86
            | FtwTarget::AndroidLinuxX86_64
            | FtwTarget::LinuxX86
            | FtwTarget::LinuxX86_64 => "so",
            FtwTarget::WindowsX86Gnu
            | FtwTarget::WindowsX86Msvc
            | FtwTarget::WindowsX86_64Gnu
            | FtwTarget::WindowsX86_64Msvc => "dll",
            FtwTarget::IosAarch64 => "a",
            FtwTarget::MacOsX86_64 => "dylib",
        }
        .to_string()
    }
}

impl ToLibPrefix for FtwTarget {
    fn to_lib_prefix(&self) -> LibPrefix {
        match self {
            FtwTarget::WindowsX86Gnu
            | FtwTarget::WindowsX86Msvc
            | FtwTarget::WindowsX86_64Gnu
            | FtwTarget::WindowsX86_64Msvc => "",
            _ => "lib",
        }
        .to_string()
    }
}

impl FromStr for FtwTarget {
    type Err = FtwError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "android-aarch64" => Ok(FtwTarget::AndroidLinuxAarch64),
            "android-arm" => Ok(FtwTarget::AndroidLinuxArmV7),
            "android-x86" => Ok(FtwTarget::AndroidLinuxX86),
            "android-x86_64" => Ok(FtwTarget::AndroidLinuxX86_64),
            "ios-aarch64" => Ok(FtwTarget::IosAarch64),
            "linux-x86" => Ok(FtwTarget::LinuxX86),
            "linux-x86_64" => Ok(FtwTarget::LinuxX86_64),
            "macos-x86_64" => Ok(FtwTarget::MacOsX86_64),
            "windows-x86-gnu" => Ok(FtwTarget::WindowsX86Gnu),
            "windows-x86" | "windows-x86-msvc" => Ok(FtwTarget::WindowsX86Msvc),
            "windows-x86_64-gnu" => Ok(FtwTarget::WindowsX86_64Gnu),
            "windows-x86_64" | "windows-x86_64-msvc" => Ok(FtwTarget::WindowsX86_64Msvc),
            _ => Err(FtwError::UnsupportedTarget),
        }
    }
}

#[cfg(test)]
mod ftw_target_tests {
    use super::*;
    use proptest::prelude::{prop_assert, prop_assume, proptest};

    #[test]
    fn test_to_cli_arg() {
        let cli_arg_targets = vec![
            ("aarch64-linux-android", FtwTarget::AndroidLinuxAarch64),
            ("armv7-linux-androideabi", FtwTarget::AndroidLinuxArmV7),
            ("i686-linux-android", FtwTarget::AndroidLinuxX86),
            ("x86_64-linux-android", FtwTarget::AndroidLinuxX86_64),
            ("aarch64-apple-ios", FtwTarget::IosAarch64),
            ("i686-unknown-linux-gnu", FtwTarget::LinuxX86),
            ("x86_64-unknown-linux-gnu", FtwTarget::LinuxX86_64),
            ("x86_64-apple-darwin", FtwTarget::MacOsX86_64),
            ("i686-pc-windows-gnu", FtwTarget::WindowsX86Gnu),
            ("i686-pc-windows-msvc", FtwTarget::WindowsX86Msvc),
            ("x86_64-pc-windows-gnu", FtwTarget::WindowsX86_64Gnu),
            ("x86_64-pc-windows-msvc", FtwTarget::WindowsX86_64Msvc),
        ];
        for (cli_arg, target) in cli_arg_targets {
            assert_eq!(cli_arg, target.to_cli_arg());
        }
    }

    #[test]
    fn test_to_export_name() {
        let export_name_targets = vec![
            ("Android", FtwTarget::AndroidLinuxAarch64),
            ("Android", FtwTarget::AndroidLinuxArmV7),
            ("Android", FtwTarget::AndroidLinuxX86),
            ("Android", FtwTarget::AndroidLinuxX86_64),
            ("iOS", FtwTarget::IosAarch64),
            ("Linux/X11", FtwTarget::LinuxX86),
            ("Linux/X11", FtwTarget::LinuxX86_64),
            ("Mac OSX", FtwTarget::MacOsX86_64),
            ("Windows", FtwTarget::WindowsX86Gnu),
            ("Windows", FtwTarget::WindowsX86Msvc),
            ("Windows", FtwTarget::WindowsX86_64Gnu),
            ("Windows", FtwTarget::WindowsX86_64Msvc),
        ];
        for (export_name, target) in export_name_targets {
            assert_eq!(export_name, target.to_export_name());
        }
    }

    #[test]
    fn test_to_app_ext() {
        let app_ext_targets = vec![
            (".apk", FtwTarget::AndroidLinuxAarch64),
            (".apk", FtwTarget::AndroidLinuxArmV7),
            (".apk", FtwTarget::AndroidLinuxX86),
            (".apk", FtwTarget::AndroidLinuxX86_64),
            (".ipa", FtwTarget::IosAarch64),
            ("", FtwTarget::LinuxX86),
            ("", FtwTarget::LinuxX86_64),
            ("", FtwTarget::MacOsX86_64),
            (".exe", FtwTarget::WindowsX86Gnu),
            (".exe", FtwTarget::WindowsX86Msvc),
            (".exe", FtwTarget::WindowsX86_64Gnu),
            (".exe", FtwTarget::WindowsX86_64Msvc),
        ];
        for (app_ext, target) in app_ext_targets {
            assert_eq!(app_ext, target.to_app_ext());
        }
    }

    #[test]
    fn test_to_lib_ext() {
        let lib_ext_targets = vec![
            ("so", FtwTarget::AndroidLinuxAarch64),
            ("so", FtwTarget::AndroidLinuxArmV7),
            ("so", FtwTarget::AndroidLinuxX86),
            ("so", FtwTarget::AndroidLinuxX86_64),
            ("a", FtwTarget::IosAarch64),
            ("so", FtwTarget::LinuxX86),
            ("so", FtwTarget::LinuxX86_64),
            ("dylib", FtwTarget::MacOsX86_64),
            ("dll", FtwTarget::WindowsX86Gnu),
            ("dll", FtwTarget::WindowsX86Msvc),
            ("dll", FtwTarget::WindowsX86_64Gnu),
            ("dll", FtwTarget::WindowsX86_64Msvc),
        ];
        for (lib_ext, target) in lib_ext_targets {
            assert_eq!(lib_ext, target.to_lib_ext());
        }
    }

    #[test]
    fn test_to_lib_prefix() {
        let lib_prefix_targets = vec![
            ("lib", FtwTarget::AndroidLinuxAarch64),
            ("lib", FtwTarget::AndroidLinuxArmV7),
            ("lib", FtwTarget::AndroidLinuxX86),
            ("lib", FtwTarget::AndroidLinuxX86_64),
            ("lib", FtwTarget::IosAarch64),
            ("lib", FtwTarget::LinuxX86),
            ("lib", FtwTarget::LinuxX86_64),
            ("lib", FtwTarget::MacOsX86_64),
            ("", FtwTarget::WindowsX86Gnu),
            ("", FtwTarget::WindowsX86Msvc),
            ("", FtwTarget::WindowsX86_64Gnu),
            ("", FtwTarget::WindowsX86_64Msvc),
        ];
        for (lib_prefix, target) in lib_prefix_targets {
            assert_eq!(lib_prefix, target.to_lib_prefix());
        }
    }

    #[test]
    fn test_from_str() -> Result<(), FtwError> {
        let from_str_targets = vec![
            ("android-aarch64", FtwTarget::AndroidLinuxAarch64),
            ("android-arm", FtwTarget::AndroidLinuxArmV7),
            ("android-x86", FtwTarget::AndroidLinuxX86),
            ("android-x86_64", FtwTarget::AndroidLinuxX86_64),
            ("ios-aarch64", FtwTarget::IosAarch64),
            ("linux-x86", FtwTarget::LinuxX86),
            ("linux-x86_64", FtwTarget::LinuxX86_64),
            ("macos-x86_64", FtwTarget::MacOsX86_64),
            ("windows-x86-gnu", FtwTarget::WindowsX86Gnu),
            ("windows-x86", FtwTarget::WindowsX86Msvc),
            ("windows-x86-msvc", FtwTarget::WindowsX86Msvc),
            ("windows-x86_64-gnu", FtwTarget::WindowsX86_64Gnu),
            ("windows-x86_64", FtwTarget::WindowsX86_64Msvc),
            ("windows-x86_64-msvc", FtwTarget::WindowsX86_64Msvc),
        ];
        for (from_str, target) in from_str_targets {
            assert_eq!(target, from_str.parse()?);
        }
        Ok(())
    }

    proptest! {
        #[test]
        fn test_from_str_error(target_input in "\\PC*") {
            let from_strs = vec![
                "android-aarch64",
                "android-arm",
                "android-x86",
                "android-x86_64",
                "ios-aarch64",
                "linux-x86",
                "linux-x86_64",
                "macos-x86_64",
                "windows-x86-gnu",
                "windows-x86",
                "windows-x86-msvc",
                "windows-x86_64-gnu",
                "windows-x86_64",
                "windows-x86_64-msvc",
            ];
            for from_str in from_strs {
                prop_assume!(target_input != from_str);
            }
            if let FtwError::UnsupportedTarget = target_input.parse::<FtwTarget>().unwrap_err() {
                 prop_assert!(true);
            }
        }
    }
}
