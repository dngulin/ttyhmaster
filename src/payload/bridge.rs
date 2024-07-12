use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePlayerRequest {
    pub player_name: String,
    pub password: HashedPassword,
}

#[derive(Deserialize)]
pub struct UpdatePlayerRequest {
    pub password: Option<HashedPassword>,
    pub is_mojang: Option<bool>,
    pub player_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HashedPassword {
    pub hash: String,
    pub salt: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfoResponse {
    pub player_id: String,
    pub is_mojang: bool,
}

#[derive(Deserialize)]
pub struct PlayerRequestQuery {
    pub name: String,
}

impl UpdatePlayerRequest {
    pub fn has_data(&self) -> bool {
        self.player_name.is_some() || self.password.is_some() || self.is_mojang.is_some()
    }
}

impl IntoResponse for PlayerInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
