use crate::ftw_error::FtwError;
use crate::traits::{ToAppExt, ToCliArg, ToExportName, ToLibExt, ToLibPrefix};
use crate::type_alias::{AppExt, CliArg, ExportName, LibExt, LibPrefix};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Default, Debug, EnumIter, Eq, Ord, PartialOrd, PartialEq)]
pub enum FtwTarget {
    AndroidLinuxAarch64,
    AndroidLinuxArmV7,
    AndroidLinuxX86,
    AndroidLinuxX86_64,
    IosAarch64,
    LinuxX86,
    LinuxX86_64,
    MacOsX86_64,
    MacOsAarch64,
    WindowsX86Gnu,
    WindowsX86Msvc,
    WindowsX86_64Gnu,
    #[default]
    WindowsX86_64Msvc,
}

#[rustfmt::skip]
impl FtwTarget {
    fn is(self, target: FtwTarget) -> bool {
        self == target
    }

    /// # Errors
    ///
    /// Will return `Err` if `target` is not Linux x86-64
    pub fn is_linux_server(self) -> Result<bool, FtwError> {
        if self.is(FtwTarget::LinuxX86_64) {
            Ok(true)
        } else {
            Err(FtwError::UnsupportedTarget)
        }
    }

    fn is_android(self) -> bool {
        matches!(self, FtwTarget::AndroidLinuxAarch64 | FtwTarget::AndroidLinuxArmV7 | FtwTarget::AndroidLinuxX86 | FtwTarget::AndroidLinuxX86_64)
    }

    fn is_windows(self) -> bool {
        matches!(self, FtwTarget::WindowsX86Gnu | FtwTarget::WindowsX86Msvc | FtwTarget::WindowsX86_64Gnu | FtwTarget::WindowsX86_64Msvc)
    }

    fn is_ios(self) -> bool {
        matches!(self, FtwTarget::IosAarch64)
    }

    fn is_linux(self) -> bool {
        matches!(self, FtwTarget::LinuxX86)
    }

    fn is_linux_x86_64(self) -> bool {
        matches!(self, FtwTarget::LinuxX86_64)
    }

    fn is_macos(self) -> bool {
        matches!(self, FtwTarget::MacOsX86_64 | FtwTarget::MacOsAarch64)
    }
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
            FtwTarget::MacOsAarch64 => "aarch64-apple-darwin",
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
        let s = self;
        match s {
            s if s.is_android() => "Android",
            s if s.is_ios() => "iOS",
            s if s.is_linux() | s.is_linux_x86_64() => "Linux/X11",
            s if s.is_macos() => "Mac OSX",
            s if s.is_windows() => "Windows Desktop",
            _ => unreachable!(),
        }
        .to_string()
    }
}

impl ToAppExt for FtwTarget {
    fn to_app_ext(&self) -> AppExt {
        let s = self;
        match s {
            s if s.is_linux() => "",
            s if s.is_linux_x86_64() => ".x86_64",
            s if s.is_android() => ".apk",
            s if s.is_macos() | s.is_ios() => ".zip",
            s if s.is_windows() => ".exe",
            _ => unreachable!(),
        }
        .to_string()
    }
}

impl ToLibExt for FtwTarget {
    fn to_lib_ext(&self) -> LibExt {
        let s = self;
        match s {
            s if s.is_android() | s.is_linux() | s.is_linux_x86_64() => "so",
            s if s.is_windows() => "dll",
            s if s.is_ios() => "a",
            s if s.is_macos() => "dylib",
            _ => unreachable!(),
        }
        .to_string()
    }
}

impl ToLibPrefix for FtwTarget {
    fn to_lib_prefix(&self) -> LibPrefix {
        let s = self;
        match s {
            s if s.is_windows() => "",
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
            "macos-aarch64" => Ok(FtwTarget::MacOsAarch64),
            "windows-x86-gnu" => Ok(FtwTarget::WindowsX86Gnu),
            "windows-x86" | "windows-x86-msvc" => Ok(FtwTarget::WindowsX86Msvc),
            "windows-x86_64-gnu" => Ok(FtwTarget::WindowsX86_64Gnu),
            "windows-x86_64" | "windows-x86_64-msvc" => Ok(FtwTarget::WindowsX86_64Msvc),
            _ => Err(FtwError::UnsupportedTarget),
        }
    }
}

impl Display for FtwTarget {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let cli_arg = self.to_cli_arg();
        write!(f, "{cli_arg}")
    }
}

#[cfg(test)]
mod ftw_target_tests {
    use super::*;
    use proptest::prelude::{prop_assert, prop_assume, proptest};

    #[test]
    fn test_to_cli_arg() {
        let cli_arg_targets = [
            ("aarch64-linux-android", FtwTarget::AndroidLinuxAarch64),
            ("armv7-linux-androideabi", FtwTarget::AndroidLinuxArmV7),
            ("i686-linux-android", FtwTarget::AndroidLinuxX86),
            ("x86_64-linux-android", FtwTarget::AndroidLinuxX86_64),
            ("aarch64-apple-ios", FtwTarget::IosAarch64),
            ("i686-unknown-linux-gnu", FtwTarget::LinuxX86),
            ("x86_64-unknown-linux-gnu", FtwTarget::LinuxX86_64),
            ("x86_64-apple-darwin", FtwTarget::MacOsX86_64),
            ("aarch64-apple-darwin", FtwTarget::MacOsAarch64),
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
        let export_name_targets = [
            ("Android", FtwTarget::AndroidLinuxAarch64),
            ("Android", FtwTarget::AndroidLinuxArmV7),
            ("Android", FtwTarget::AndroidLinuxX86),
            ("Android", FtwTarget::AndroidLinuxX86_64),
            ("iOS", FtwTarget::IosAarch64),
            ("Linux/X11", FtwTarget::LinuxX86),
            ("Linux/X11", FtwTarget::LinuxX86_64),
            ("Mac OSX", FtwTarget::MacOsX86_64),
            ("Mac OSX", FtwTarget::MacOsAarch64),
            ("Windows Desktop", FtwTarget::WindowsX86Gnu),
            ("Windows Desktop", FtwTarget::WindowsX86Msvc),
            ("Windows Desktop", FtwTarget::WindowsX86_64Gnu),
            ("Windows Desktop", FtwTarget::WindowsX86_64Msvc),
        ];
        for (export_name, target) in export_name_targets {
            assert_eq!(export_name, target.to_export_name());
        }
    }

    #[test]
    fn test_to_app_ext() {
        let app_ext_targets = [
            (".apk", FtwTarget::AndroidLinuxAarch64),
            (".apk", FtwTarget::AndroidLinuxArmV7),
            (".apk", FtwTarget::AndroidLinuxX86),
            (".apk", FtwTarget::AndroidLinuxX86_64),
            (".zip", FtwTarget::IosAarch64),
            ("", FtwTarget::LinuxX86),
            (".x86_64", FtwTarget::LinuxX86_64),
            (".zip", FtwTarget::MacOsX86_64),
            (".zip", FtwTarget::MacOsAarch64),
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
        let lib_ext_targets = [
            ("so", FtwTarget::AndroidLinuxAarch64),
            ("so", FtwTarget::AndroidLinuxArmV7),
            ("so", FtwTarget::AndroidLinuxX86),
            ("so", FtwTarget::AndroidLinuxX86_64),
            ("a", FtwTarget::IosAarch64),
            ("so", FtwTarget::LinuxX86),
            ("so", FtwTarget::LinuxX86_64),
            ("dylib", FtwTarget::MacOsX86_64),
            ("dylib", FtwTarget::MacOsAarch64),
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
        let lib_prefix_targets = [
            ("lib", FtwTarget::AndroidLinuxAarch64),
            ("lib", FtwTarget::AndroidLinuxArmV7),
            ("lib", FtwTarget::AndroidLinuxX86),
            ("lib", FtwTarget::AndroidLinuxX86_64),
            ("lib", FtwTarget::IosAarch64),
            ("lib", FtwTarget::LinuxX86),
            ("lib", FtwTarget::LinuxX86_64),
            ("lib", FtwTarget::MacOsX86_64),
            ("lib", FtwTarget::MacOsAarch64),
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
        let from_str_targets = [
            ("android-aarch64", FtwTarget::AndroidLinuxAarch64),
            ("android-arm", FtwTarget::AndroidLinuxArmV7),
            ("android-x86", FtwTarget::AndroidLinuxX86),
            ("android-x86_64", FtwTarget::AndroidLinuxX86_64),
            ("ios-aarch64", FtwTarget::IosAarch64),
            ("linux-x86", FtwTarget::LinuxX86),
            ("linux-x86_64", FtwTarget::LinuxX86_64),
            ("macos-x86_64", FtwTarget::MacOsX86_64),
            ("macos-aarch64", FtwTarget::MacOsAarch64),
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

    #[test]
    fn test_is_linux_x86_64() -> Result<(), FtwError> {
        let targets = [
            (FtwTarget::AndroidLinuxAarch64),
            (FtwTarget::AndroidLinuxArmV7),
            (FtwTarget::AndroidLinuxX86),
            (FtwTarget::AndroidLinuxX86_64),
            (FtwTarget::IosAarch64),
            (FtwTarget::LinuxX86),
            (FtwTarget::LinuxX86_64),
            (FtwTarget::MacOsX86_64),
            (FtwTarget::MacOsAarch64),
            (FtwTarget::WindowsX86Gnu),
            (FtwTarget::WindowsX86Msvc),
            (FtwTarget::WindowsX86Msvc),
            (FtwTarget::WindowsX86_64Gnu),
            (FtwTarget::WindowsX86_64Msvc),
            (FtwTarget::WindowsX86_64Msvc),
        ];
        for target in targets {
            if target == FtwTarget::LinuxX86_64 {
                assert!(target.is_linux_server().unwrap());
            } else {
                let err = target.is_linux_server().unwrap_err();
                match err {
                    FtwError::UnsupportedTarget => assert!(true),
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_fmt() {
        let cli_arg_targets = [
            ("aarch64-linux-android", FtwTarget::AndroidLinuxAarch64),
            ("armv7-linux-androideabi", FtwTarget::AndroidLinuxArmV7),
            ("i686-linux-android", FtwTarget::AndroidLinuxX86),
            ("x86_64-linux-android", FtwTarget::AndroidLinuxX86_64),
            ("aarch64-apple-ios", FtwTarget::IosAarch64),
            ("i686-unknown-linux-gnu", FtwTarget::LinuxX86),
            ("x86_64-unknown-linux-gnu", FtwTarget::LinuxX86_64),
            ("x86_64-apple-darwin", FtwTarget::MacOsX86_64),
            ("aarch64-apple-darwin", FtwTarget::MacOsAarch64),
            ("i686-pc-windows-gnu", FtwTarget::WindowsX86Gnu),
            ("i686-pc-windows-msvc", FtwTarget::WindowsX86Msvc),
            ("x86_64-pc-windows-gnu", FtwTarget::WindowsX86_64Gnu),
            ("x86_64-pc-windows-msvc", FtwTarget::WindowsX86_64Msvc),
        ];
        for (cli_arg, target) in cli_arg_targets {
            assert_eq!(cli_arg, format!("{target}"));
        }
    }

    #[test]
    fn test_default() {
        assert_eq!(FtwTarget::default(), FtwTarget::WindowsX86_64Msvc);
    }

    proptest! {
        #[test]
        fn test_from_str_error(target_input in "\\PC*") {
            let from_strs = [
                "android-aarch64",
                "android-arm",
                "android-x86",
                "android-x86_64",
                "ios-aarch64",
                "linux-x86",
                "linux-x86_64",
                "macos-x86_64",
                "macos-aarch64",
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
