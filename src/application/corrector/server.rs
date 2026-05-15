use std::fs::exists;
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

pub fn is_ready() -> ServerReadyResult {
    let environment_status = check_environment_file();

    if !environment_status {
        return ServerReadyResult::Failed(
            "The environment file doesn't exist".to_string()
        )
    }

    ServerReadyResult::Success

}