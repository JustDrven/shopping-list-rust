use std::fs::exists;
use std::time::Duration;

use pinger::{PingOptions, Pinger};
use pinger::linux::LinuxPinger;
use pinger::macos::MacOSPinger;

use crate::application;
use crate::application::enums::logger::LoggerType;
use crate::application::enums::server::ServerReadyResult;
use crate::application::util::logger;

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

fn check_port() -> bool {
    let current_os: String = application::util::os::current();
    let ping_option: PingOptions = PingOptions::new_ipv4(
        application::util::address::create_address(),
        Duration::from_secs(3),
        Some("eth0".to_string()),
    );

    if current_os.eq(application::util::os::MACOS) {
        macos_pinger(ping_option)
    } else if current_os.eq(application::util::os::LINUX) {
        linux_pinger(ping_option)
    } else {
        logger::log(LoggerType::Error, format!("Invalid OS: {}", current_os));
        false
    }

}

fn linux_pinger(option: PingOptions) -> bool {
    let pinger: LinuxPinger = LinuxPinger::from_options(option).unwrap();
    let response = pinger.start().unwrap().recv();

    match response {
        Ok(_) => {true},
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
