use crate::db::Stats;
use crate::util;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,

    #[serde(rename = "launcherVersion")]
    pub launcher_version: String,
    pub platform: LauncherPlatformInfo,

    #[serde(rename = "ticket")]
    pub install_uuid: String,
    #[serde(rename = "uuid")]
    pub machine_uuid: String,
}

#[derive(Deserialize)]
pub struct LauncherPlatformInfo {
    pub os: String,
    #[serde(rename = "version")]
    pub os_version: String,
    #[serde(rename = "word")]
    pub word_size: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    #[serde(rename = "clientToken")]
    pub player_id: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

impl LoginRequest {
    pub fn validate(&self) -> bool {
        self.platform.validate()
            && util::str_has_valid_len(&self.username, 32)
            && util::str_has_valid_len(&self.password, 32)
            && util::str_has_valid_len(&self.launcher_version, 32)
            && Uuid::try_parse(&self.install_uuid).is_ok()
            && Uuid::try_parse(&self.machine_uuid).is_ok()
    }
}

impl LauncherPlatformInfo {
    pub fn validate(&self) -> bool {
        util::str_has_valid_len(&self.os_version, 64)
            && ["linux", "osx", "windows"].contains(&self.os.as_str())
            && ["32", "64"].contains(&self.word_size.as_str())
    }
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<'a> From<&'a LoginRequest> for Stats<'a> {
    fn from(value: &'a LoginRequest) -> Self {
        Stats {
            player_name: &value.username,
            launcher_version: &value.launcher_version,
            os: &value.platform.os,
            os_version: &value.platform.os_version,
            os_word_size: &value.platform.word_size,
            install_uuid: &value.install_uuid,
            machine_uuid: &value.machine_uuid,
        }
    }
}
