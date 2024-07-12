use crate::util;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct JoinRequest {
    #[serde(rename = "selectedProfile")]
    pub player_id_compact: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Deserialize)]
pub struct HasJoinedRequestQuery {
    pub username: String,
    #[serde(rename = "serverId")]
    pub server_id: String,
}

impl JoinRequest {
    pub fn validate(&self) -> bool {
        util::str_has_valid_len(&self.server_id, 64)
            && Uuid::try_parse(&self.player_id_compact).is_ok()
            && Uuid::try_parse(&self.access_token).is_ok()
    }
}

impl HasJoinedRequestQuery {
    pub fn validate(&self) -> bool {
        util::str_has_valid_len(&self.username, 32) && util::str_has_valid_len(&self.server_id, 64)
    }
}
