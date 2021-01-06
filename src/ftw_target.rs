use crate::ftw_error::FtwError;
use crate::traits::{ToCliArg, ToLibExt, ToLibPrefix};
use crate::type_alias::{CliArg, LibExt, LibPrefix};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq)]
pub enum FtwTarget {
    AndroidLinuxAarch64,
    AndroidLinuxArmV7,
    AndroidLinuxX86,
    AndroidLinuxX86_64,
    IosAarch64,
    IosX64_64,
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
            FtwTarget::IosX64_64 => "x86_64-apple-ios",
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
            FtwTarget::IosAarch64 | FtwTarget::IosX64_64 | FtwTarget::MacOsX86_64 => "dylib",
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
            "ios-x86_64" => Ok(FtwTarget::IosX64_64),
            "linux-x86" => Ok(FtwTarget::LinuxX86),
            "linux-x86_64" => Ok(FtwTarget::LinuxX86_64),
            "macos-x86_64" => Ok(FtwTarget::MacOsX86_64),
            "windows-x86-gnu" => Ok(FtwTarget::WindowsX86Gnu),
            "windows-x86-msvc" => Ok(FtwTarget::WindowsX86Msvc),
            "windows-x86" => Ok(FtwTarget::WindowsX86Msvc),
            "windows-x86_64-gnu" => Ok(FtwTarget::WindowsX86_64Gnu),
            "windows-x86_64-msvc" => Ok(FtwTarget::WindowsX86_64Msvc),
            "windows-x86_64" => Ok(FtwTarget::WindowsX86_64Msvc),
            _ => Err(FtwError::UnsupportedTarget),
        }
    }
}
