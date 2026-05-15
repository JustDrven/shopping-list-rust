use std::collections::HashMap;
use std::ptr::null;
use orma::Connection;
use crate::application::enums::logger::LoggerType;
use crate::application::util::logger;

pub fn initialize(variables: &mut HashMap<String, String>) -> Result<bool, String> {

    let host_key = "DB_HOST";
    let username_key = "DB_USERNAME";
    let password_key = "DB_PASSWORD";
    let db_key = "DB_NAME";

    if !variables.contains_key(host_key) {
        return Err("Missing host variable".to_string());
    }

    if !variables.contains_key(username_key) {
        return Err("Missing username variable".to_string());
    }

    if !variables.contains_key(password_key) {
        return Err("Missing password variable".to_string());
    }

    if !variables.contains_key(db_key) {
        return Err("Missing database name variable".to_string());
    }

    let host = variables.get(host_key).unwrap();
    let username = variables.get(username_key).unwrap();
    let password = variables.get(password_key).unwrap();
    let db_name = variables.get(db_key).unwrap();

    logger::log(LoggerType::Info, format!("Opening database connection for {} - password: {}", host, !password.is_empty()));
    let connection = create_connection(
        host, username, password, db_name
    );

    Ok(true)
}

fn create_connection(host: &String, username: &String, password: &String, db_name: &String) -> bool {

    todo!("Add connection to database");
    true

}