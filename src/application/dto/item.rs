use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ItemDto {
    pub id: i32,
    pub name: String,
    pub complete: bool,
}
