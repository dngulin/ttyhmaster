use crate::db::PlayerAuthRecord;
use crate::payload::TexturesProperty;
use axum::http::HeaderMap;
use base64::Engine;
use sha1::{Digest, Sha1};
use std::io::ErrorKind;
use std::path::Path;
use uuid::Uuid;

use base64::engine::general_purpose::STANDARD as BASE64;
use tokio::fs;

pub fn str_has_valid_len(value: &str, max_len: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max_len
}

pub fn check_password(password: &str, credentials: &PlayerAuthRecord) -> bool {
    credentials.password_hash == hash_with_salt(password, &credentials.salt)
}

pub fn hash_with_salt(password: &str, salt: &str) -> String {
    let raw_hash = format!("{:x}", Sha1::digest(password));

    let mut sha = Sha1::new();
    sha.update(salt);
    sha.update(raw_hash);

    format!("{:x}", sha.finalize())
}

pub fn to_hyphenated_uuid(uuid_simple: &str) -> Option<String> {
    let uuid = Uuid::try_parse(uuid_simple).ok()?;
    Some(uuid.hyphenated().to_string())
}

pub fn to_compact_uuid(uuid: &str) -> Option<String> {
    let uuid = Uuid::try_parse(uuid).ok()?;
    Some(uuid.simple().to_string())
}

pub async fn file_exists(path: &Path) -> bool {
    fs::metadata(path)
        .await
        .map_err(|error| match error.kind() {
            ErrorKind::NotFound => (),
            _ => {
                let path = path.display();
                tracing::error!("Check if file exists `{path}`: {error}");
            }
        })
        .is_ok()
}

pub fn serialize_textures_base64(textures: &TexturesProperty) -> anyhow::Result<String> {
    let json = serde_json::to_string(textures).map_err(|error| {
        tracing::error!("Serialize textures: {error}");
        error
    })?;

    Ok(BASE64.encode(json))
}

pub fn get_access_token_from_header(headers: &HeaderMap) -> Option<&str> {
    const AUTH_HEADER: &str = "Authorization";
    const TOKEN_PREFIX: &str = "Bearer ";

    headers.get(AUTH_HEADER).and_then(|header| {
        let value = header.to_str().ok()?;
        if value.len() > TOKEN_PREFIX.len() && value.starts_with(TOKEN_PREFIX) {
            Some(&value[TOKEN_PREFIX.len()..])
        } else {
            None
        }
    })
}
