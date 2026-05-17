use crate::application::enums::logger::LoggerType;

pub fn log(log_type: LoggerType, message: String) {
    let prefix = get_prefix(log_type);
    
    println!("[{}] {}", prefix, message);
}


fn get_prefix(logger_type: LoggerType) -> String {
    match logger_type {
        LoggerType::Info => String::from("INFO"),
        LoggerType::Error => String::from("ERROR"),
        LoggerType::Warning => String::from("WARNING"),
    }
}


