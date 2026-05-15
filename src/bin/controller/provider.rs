use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::bin::controller;

pub fn initialize_controllers() -> Router {
    Router::new()
        .route("/monitor/status", get(controller::query::status::monitor))

        .route("/items/{id}", get(controller::query::item::item))
        .route("/items", get(controller::query::item::items))

        .route("/items/{id}/complete", put(controller::mutation::item::complete))
        .route("/items", post(controller::mutation::item::create))
        .route("/items", delete(controller::mutation::item::delete))
}