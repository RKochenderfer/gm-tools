use axum::{response::IntoResponse, Json};

use crate::models::default_response::DefaultResponse;

pub async fn ping() -> impl IntoResponse {
	let res = DefaultResponse { ok: true, ..Default::default()};

    Json(res)
}