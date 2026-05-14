use axum::{
    Router, routing::get,
};

mod util;
mod controller;
mod entity;
mod dto;
mod service;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/status", get(controller::status::get_status))
        .merge(controller::provider::initialize_controllers());

    let address = util::address::create_address();
    let listener = tokio::net::TcpListener::bind(address.to_string()).await.unwrap();
    
    println!("The server is listening at {}", address.to_string());
    
    axum::serve(listener, app).await.unwrap();
}