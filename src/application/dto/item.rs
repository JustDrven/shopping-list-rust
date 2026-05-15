use serde::Serialize;

#[derive(Serialize)]
pub struct ItemDto {
    pub id: u32,
    pub name: String,
}