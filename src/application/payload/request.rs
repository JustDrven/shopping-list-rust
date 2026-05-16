use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CompleteRequest(pub u32);
#[derive(Deserialize)]
pub struct DeleteRequest(pub u32);
#[derive(Deserialize)]
pub struct CreateRequest(pub String);
