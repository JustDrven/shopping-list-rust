
use axum::{
    routing::get,
    Router,
};

mod util;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Ahoj z Axumu!" }));

    let port = util::constant::PORT;

    let listener = tokio::net::TcpListener::bind("").await.unwrap();
    println!("Server is listening at {}", port);
    
    axum::serve(listener, app).await.unwrap();
}