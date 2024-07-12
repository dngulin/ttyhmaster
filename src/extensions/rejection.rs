use crate::payload::ErrorResponse;
use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Rejection))]
pub struct ReJson<T>(pub T);

pub struct Rejection;

impl From<JsonRejection> for Rejection {
    fn from(rejection: JsonRejection) -> Self {
        tracing::debug!("Reject payload: {rejection}");
        Self
    }
}

impl IntoResponse for Rejection {
    fn into_response(self) -> Response {
        ErrorResponse::bad_request().into_response()
    }
}
