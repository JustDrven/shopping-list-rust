use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OkResponse {
    ok: bool,
    message: String,
}

pub fn ok_response(ok: bool, message: String) -> OkResponse {
    OkResponse{ ok, message }
}



