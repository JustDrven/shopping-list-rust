use std::env::consts::OS;
use std::string::ToString;

const MACOS: &str = "macos";
const LINUX: &str = "linux";

pub fn current() -> String {
    OS.to_string()
}

pub fn is_linux() -> bool {
    current().eq(LINUX)
}

pub fn is_macos() -> bool {
    current().eq(MACOS)
}


