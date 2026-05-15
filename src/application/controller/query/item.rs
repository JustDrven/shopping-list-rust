use axum::Json;
use crate::application::dto::item::ItemDto;

pub async fn item() -> Json<ItemDto> {

    let body: ItemDto = ItemDto {
        name: "jidlo".to_string(),
        id: 94373
    };

    Json(body)
}

pub async fn items() -> String {
    "Hello".to_string()
}

