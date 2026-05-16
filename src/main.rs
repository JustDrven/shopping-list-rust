use axum::Router;
use tokio::net::TcpListener;

use application::enums::logger::LoggerType;
use application::enums::server::ServerReadyResult;
use application::util::logger;

pub mod application;

async fn create_server() -> TcpListener {
    let address: String = application::util::address::create_address();
    let listener: TcpListener = TcpListener::bind(address.to_string()).await.unwrap();

    logger::log(LoggerType::Info, format!("The server is listening at {}", address));

    listener
}

async fn load_databases() -> bool {
    let mut variables = application::environment::loader::load().await;
    let result = application::orm::database::initialize(
        &mut variables
    ).await;

    match result {
        Ok(_) => true,
        Err(error) => {
            let error_message: String = format!("An error occurred while loading the database: {}", error);
            logger::log(LoggerType::Info, error_message);
            false
        }
    }
}

#[tokio::main]
async fn main() {
    let server_status: ServerReadyResult = application::corrector::server::is_ready();
    match server_status {
        ServerReadyResult::Failed(err) => {
            let error_message: String = format!("Sorry, but there is an error with '{}'", err);
            logger::log(LoggerType::Error, error_message);
            return;
        }

        ServerReadyResult::Success => {
            logger::log(LoggerType::Info, "Everything is good, we are ready to start server!".to_string());
        }
    }

    if !load_databases().await {
        logger::log(LoggerType::Error, "Failed to load databases".to_string());
        return;
    };

    let app: Router = application::controller::provider::create_router();
    let listener: TcpListener = create_server().await;
    
    axum::serve(listener, app).await.unwrap();
}