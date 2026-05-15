use axum::Router;
use crate::bin::enums::server::ServerReadyResult;

pub mod bin;

#[tokio::main]
async fn main() {
    let server_status: ServerReadyResult = bin::corrector::server::is_ready();
    match server_status {
        ServerReadyResult::Failed(err) => {
            println!("Sorry, but there is an error with '{}'", err);
            return;
        }

        ServerReadyResult::Success => {
            println!("Everything is good, we are ready to start server!");
        }
    }

    let app = Router::new()
        .merge(bin::controller::provider::initialize_controllers());

    let address = bin::util::address::create_address();
    let listener = tokio::net::TcpListener::bind(address.to_string()).await.unwrap();
    
    println!("The server is listening at {}", address.to_string());
    
    axum::serve(listener, app).await.unwrap();
}