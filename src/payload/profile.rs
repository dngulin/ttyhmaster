use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProfileRequestQuery {
    pub unsigned: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerProfile {
    #[serde(rename = "id")]
    pub player_id: String,
    #[serde(rename = "name")]
    pub player_name: String,
    pub properties: Vec<SerializedProperty>,
}

impl IntoResponse for PlayerProfile {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializedProperty {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}
