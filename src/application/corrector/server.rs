use std::fs::exists;
use std::time::Duration;

use pinger::{
    PingOptions, Pinger
};
use pinger::linux::LinuxPinger;
use pinger::macos::MacOSPinger;

use crate::application;
use crate::application::enums::logger::LoggerType;
use crate::application::enums::server::ServerReadyResult;
use crate::application::util::logger;
use crate::application::util::os;

fn check_environment_file() -> bool {
    let file_status = exists("./.env");
    match file_status {
        Ok(_) => {
            true
        }

        Err(err) => {
            logger::log(LoggerType::Error, format!("Specific Error: {}", err));
            false
        }
    }
}

fn get_ping_options() -> PingOptions {
    let address: String = application::util::address::create_address();
    let timeout: Duration = Duration::from_secs(3);
    let interface: Option<String> = Some("eth0".to_string());

    PingOptions::new_ipv4(address, timeout, interface)
}

fn check_port() -> bool {
    let ping_option: PingOptions = get_ping_options();

    if os::is_macos() {
        macos_pinger(ping_option)
    } else if os::is_linux() {
        linux_pinger(ping_option)
    } else {
        logger::log(LoggerType::Error, format!("Invalid OS: {}", os::current()));
        false
    }

}

fn linux_pinger(option: PingOptions) -> bool {
    let pinger: LinuxPinger = LinuxPinger::from_options(option).unwrap();
    let response = pinger.start().unwrap().recv();

    match response {
        Ok(_) => { true },
        Err(err) => {
            logger::log(LoggerType::Error, format!("Linux Error: {}", err));
            false
        }
    }

}

fn macos_pinger(option: PingOptions) -> bool {
    let pinger: MacOSPinger = MacOSPinger::from_options(option).unwrap();
    let response = pinger.start().unwrap().recv();

    match response {
        Ok(_) => {true},
        Err(err) => {
            logger::log(LoggerType::Error, format!("MacOS Error: {}", err));
            false
        }
    }

}

pub fn is_ready() -> ServerReadyResult {
    let environment_status: bool = check_environment_file();
    if !environment_status {
        return ServerReadyResult::Failed(
            "The environment file doesn't exist".to_string()
        )
    }

    let port_status: bool = check_port();
    if !port_status {
        return ServerReadyResult::Failed(
            "The port is already in use".to_string()
        )
    }

    ServerReadyResult::Success
}
