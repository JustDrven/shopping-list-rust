use crate::application::dto::item::ItemDto;
use crate::application::models::item::Model;

pub fn map_item_to_dto(entity: Model) -> ItemDto {
    ItemDto {
        id: entity.id,
        name: entity.name,
        complete: entity.complete
    }
}

