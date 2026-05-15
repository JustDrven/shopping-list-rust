use std::fs::exists;
use crate::bin::enums::server::ServerReadyResult;

fn check_environment_file() -> bool {
    let file_status = exists("./.env");
    match file_status {
        Ok(_) => {
            true
        }
        Err(_) => {
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