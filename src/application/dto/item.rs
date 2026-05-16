use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ItemDto {
    pub id: u32,
    pub name: String,
}
