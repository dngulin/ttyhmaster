use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    status: &'static str,
}

impl IntoResponse for StatusResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl StatusResponse {
    pub fn accepted() -> Self {
        Self { status: "accepted" }
    }
}
