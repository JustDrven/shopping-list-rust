use axum::Json;

use crate::application::dto;

pub async fn monitor() -> Json<dto::monitor::Monitor> {
    Json(dto::monitor::Monitor {
        ok: true,
    })
}