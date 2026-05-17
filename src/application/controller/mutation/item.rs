use axum::Json;

use crate::application::repository;
use crate::application::payload::response;
use crate::application::payload::request;

pub async fn complete(Json(data): Json<request::CompleteRequest>) -> Json<response::OkResponse> {
    Json(repository::item::mutation::complete(data.into()).await.into())
}

pub async fn create(Json(data): Json<request::CreateRequest>) -> Json<response::OkResponse> {
    Json(repository::item::mutation::create(data.into()).await.into())
}

pub async fn delete(Json(data): Json<request::DeleteRequest>) -> Json<response::OkResponse> {
    Json(repository::item::mutation::delete(data.into()).await.into())
}


