use axum::Router;
use axum::routing::{
    delete, get, post, put
};

use crate::application::controller::query;
use crate::application::controller::mutation;
use crate::application::enums::logger::LoggerType;
use crate::application::util::logger;

pub fn create_router() -> Router {
    logger::log(LoggerType::Info, "Starting to create router!".to_string());

    Router::new()
        .route("/monitor/status", get(query::status::monitor))

        .route("/items/{id}", get(query::item::item))
        .route("/items", get(query::item::items))

        .route("/items/{id}/complete", put(mutation::item::complete))
        .route("/items", post(mutation::item::create))
        .route("/items", delete(mutation::item::delete))
}
