use axum::Json;

use crate::application::dto::item::ItemDto;

pub async fn item() -> Json<ItemDto> {

    let body: ItemDto = ItemDto {
        name: "jidlo".to_string(),
        id: 94373
    };

    Json(body)
}

pub async fn items() -> Json<Vec<ItemDto>> {
    Json(vec![ItemDto{
        name: "wwe".to_string(),
        id: 9834
    }])
}

