use crate::db::PlayerRecord;
use crate::payload::{PlayerProfile, SerializedProperty, TexturesProperty};
use crate::state::AppState;
use crate::{assets, signing, util};
use chrono::Utc;

pub async fn get(state: &AppState, player: PlayerRecord, signed: bool) -> Option<PlayerProfile> {
    let textures = TexturesProperty {
        timestamp: Utc::now().timestamp_millis(),
        player_id: player.player_id.clone(),
        player_name: player.player_name.clone(),
        signature_required: signed.then_some(true),
        textures: assets::get_textures_info(
            &state.assets_cfg,
            &player.player_id,
            player.is_slim_model,
        )
        .await,
    };

    let tex_value = util::serialize_textures_base64(&textures).ok()?;
    let tex_signature = match signed {
        true => Some(signing::sign_base64(&state.signing, tex_value.as_bytes()).ok()?),
        false => None,
    };

    let textures_property = SerializedProperty {
        name: "textures".into(),
        value: tex_value,
        signature: tex_signature,
    };

    let profile = PlayerProfile {
        player_id: util::to_compact_uuid(&player.player_id)?,
        player_name: player.player_name,
        properties: Vec::from([textures_property]),
    };

    Some(profile)
}
