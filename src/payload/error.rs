use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub struct ErrorResponse {
    status_code: StatusCode,
    message: Option<&'static str>,
}

#[derive(Serialize)]
struct ErrorResponsePayload<'a> {
    error: &'a str,
    #[serde(rename = "errorMessage")]
    message: &'a str,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let error = self
            .status_code
            .canonical_reason()
            .unwrap_or(self.status_code.as_str());

        let message = self.message.unwrap_or(error);
        let payload = ErrorResponsePayload { error, message };

        (self.status_code, Json(payload)).into_response()
    }
}

impl ErrorResponse {
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            status_code,
            message: None,
        }
    }

    pub fn new_with_msg(status_code: StatusCode, message: &'static str) -> Self {
        Self {
            status_code,
            message: Some(message),
        }
    }

    pub fn bad_request() -> Self {
        Self::new_with_msg(StatusCode::BAD_REQUEST, "Invalid request payload")
    }

    pub fn bad_request_with_msg(message: &'static str) -> Self {
        Self::new_with_msg(StatusCode::BAD_REQUEST, message)
    }

    pub fn internal_error() -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn unauthorized() -> Self {
        Self::new_with_msg(StatusCode::UNAUTHORIZED, "Invalid username or password")
    }

    pub fn forbidden_with_msg(message: &'static str) -> Self {
        Self::new_with_msg(StatusCode::FORBIDDEN, message)
    }
}
