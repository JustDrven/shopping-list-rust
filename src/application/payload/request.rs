use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CompleteRequest {
    pub id: i32,
}


#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}


#[derive(Deserialize)]
pub struct CreateRequest {
    pub name: String,
}



