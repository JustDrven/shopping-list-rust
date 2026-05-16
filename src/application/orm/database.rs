use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use crate::application::enums::logger::LoggerType;
use crate::application::util::logger;

static DB_CLIENT: OnceLock<Arc<Client>> = OnceLock::new();

pub async fn initialize(variables: &mut HashMap<String, String>) -> Result<bool, String> {

    let host_key = "DB_HOST";
    let port_key = "DB_PORT";
    let username_key = "DB_USERNAME";
    let password_key = "DB_PASSWORD";
    let db_key = "DB_NAME";

    if !variables.contains_key(host_key) {
        return Err("Missing host variable".to_string());
    }

    if !variables.contains_key(port_key) {
        return Err("Missing port variable".to_string());
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

    let host: &String = variables.get(host_key).unwrap();
    let port: &String = variables.get(port_key).unwrap();
    let username: &String = variables.get(username_key).unwrap();
    let password: &String = variables.get(password_key).unwrap();
    let db_name: &String = variables.get(db_key).unwrap();

    {
        logger::log(LoggerType::Info, format!("Opening database connection for {} - password: {}", host, !password.is_empty()));
        let new_pool = create_connection(host, port, db_name, username, password).await?;

        if DB_CLIENT.set(Arc::new(new_pool)).is_err() {
            return Err("Database already initialized".to_string());
        }
    }


    Ok(true)
}

pub fn get_pool() -> Option<Arc<Client>> {
    DB_CLIENT.get().cloned()
}

use tokio_postgres::{Client, NoTls};

async fn create_connection(
    host: &str,
    port: &str,
    db_name: &str,
    username: &str,
    password: &str) -> Result<Client, String> {
    let connection_string = format!(
        "host={} port={} dbname={} user={} password={}",
        host, port, db_name, username, password
    );


    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}


