use std::collections::HashMap;
use std::sync::{
    OnceLock
};
use sea_orm::{Database, DatabaseConnection, Iden};

use crate::application::enums::logger::LoggerType;
use crate::application::util::logger;

static DB_CLIENT: OnceLock<DatabaseConnection> = OnceLock::new();


pub fn get_pool() -> Option<DatabaseConnection> {
    let result: Option<&DatabaseConnection> = DB_CLIENT.get();

    match result {
        Some(db) => {
            let pool: DatabaseConnection = db.clone();
            Some(pool)
        },
        None => None
    }
}

pub fn validate(key: &str, variables: HashMap<String, String>) -> bool {
    !variables.contains_key(key)
}

pub async fn initialize(variables: &mut HashMap<String, String>) -> Result<bool, String> {
    let host_key: &str = "DB_HOST";
    let port_key: &str = "DB_PORT";
    let username_key: &str = "DB_USERNAME";
    let password_key: &str = "DB_PASSWORD";
    let db_key: &str = "DB_NAME";



    if !validate(host_key, variables.clone()) {
        return Err("Missing host variable".to_string());
    }

    if !validate(host_key, variables.clone()) {
        return Err("Missing port variable".to_string());
    }

    if !validate(host_key, variables.clone()) {
        return Err("Missing username variable".to_string());
    }

    if !validate(host_key, variables.clone()) {
        return Err("Missing password variable".to_string());
    }

    if !validate(host_key, variables.clone()) {
        return Err("Missing database name variable".to_string());
    }



    let host: &String = variables.get(host_key).unwrap();
    let port: &String = variables.get(port_key).unwrap();
    let username: &String = variables.get(username_key).unwrap();
    let password: &String = variables.get(password_key).unwrap();
    let db_name: &String = variables.get(db_key).unwrap();



    logger::log(LoggerType::Info, format!("Opening database connection for {} - password: {}", host, !password.is_empty()));
    let new_pool: DatabaseConnection = create_connection(
        host, port, db_name, username, password
    ).await?;

    if DB_CLIENT.set(new_pool.into()).is_err() {
        return Err("Database already initialized".to_string());
    }

    Ok(true)
}

async fn create_connection(
    host: &str, port: &str,
    db_name: &str,
    username: &str, password: &str) -> Result<DatabaseConnection, String> {

    let stream: String = format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, db_name);

    Ok(Database::connect(stream).await.unwrap())

}

