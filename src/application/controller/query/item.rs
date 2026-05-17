use axum::extract::Path;
use axum::Json;

use crate::application::dto::item::ItemDto;
use crate::application::repository;

pub async fn item(id: Path<i32>) -> Json<ItemDto> {
    Json(repository::item::query::get_item(id).await)
}

pub async fn items() -> Json<Vec<ItemDto>> {
    Json(repository::item::query::get_items().await)
}

