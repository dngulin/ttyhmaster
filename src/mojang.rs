use crate::db::PlayerRecord;
use crate::payload::PlayerProfile;
use axum::http::StatusCode;

const HAS_JOINED_PATH: &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined";

pub async fn has_joined(
    player: PlayerRecord,
    server_id: &str,
) -> Result<PlayerProfile, StatusCode> {
    let username = &player.player_name;
    let url = format!("{HAS_JOINED_PATH}?username={username}&serverId={server_id}");

    let response = reqwest::get(url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !response.status().is_success() {
        return Err(response.status());
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut player_profile: PlayerProfile =
        serde_json::from_slice(bytes.as_ref()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    player_profile.player_id = player.player_id;
    Ok(player_profile)
}
