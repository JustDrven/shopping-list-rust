use std::time::Duration;
use moka::future::Cache;

use crate::application::dto::item::ItemDto;

pub fn create_item() -> Cache<String, ItemDto> {
    Cache::builder()
        .max_capacity(1_000)
        .time_to_live(Duration::from_secs(10))
        .build()

}

pub fn create_items() -> Cache<u8, Vec<ItemDto>> {
    Cache::builder()
        .max_capacity(1_000)
        .time_to_live(Duration::from_secs(10))
        .build()

}
