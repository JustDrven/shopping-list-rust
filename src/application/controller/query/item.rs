use axum::Json;
use crate::application::dto;

pub async fn item() -> Json<dto::item::ItemDto> {
    Json(dto::item::ItemDto {
        name: "".to_string(),
        id: 0,
    })
}

pub async fn items() -> String {
    "Hello".to_string()
}

