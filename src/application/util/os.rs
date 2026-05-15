use std::env::consts::OS;
use std::string::ToString;

pub const MACOS: &str = "macos";
pub const LINUX: &str = "linux";

pub fn current() -> String {
    OS.to_string()
}