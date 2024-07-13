use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use uuid::Uuid;

use crate::db::NewPlayerInfo;
use crate::extensions::ReJson;
use crate::payload::{
    CreatePlayerRequest, ErrorResponse, HasJoinedRequestQuery, JoinRequest, LoginRequest,
    LoginResponse, PlayerCertificates, PlayerInfoResponse, PlayerProfile, ProfileRequestQuery,
    SetSkinRequest, StatusResponse, UpdatePlayerRequest,
};
use crate::state::AppState;
use crate::{assets, db, mojang, player_profile, signing, util};

pub async fn create_player(
    State(state): State<AppState>,
    headers: HeaderMap,
    ReJson(request): ReJson<CreatePlayerRequest>,
) -> Result<StatusCode, StatusCode> {
    validate_api_token(&state, &headers).or(Err(StatusCode::FORBIDDEN))?;

    let new_player_info = NewPlayerInfo {
        player_name: &request.player_name,
        player_id: &Uuid::new_v4().hyphenated().to_string(),
        password_hash: &request.password.hash,
        salt: &request.password.salt,
        access_token: &Uuid::new_v4().hyphenated().to_string(),
    };
    db::create_player(&state.db_pool, new_player_info).await?;

    Ok(StatusCode::OK)
}

pub async fn update_player(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(player_name): Path<String>,
    ReJson(request): ReJson<UpdatePlayerRequest>,
) -> Result<StatusCode, StatusCode> {
    validate_api_token(&state, &headers).or(Err(StatusCode::FORBIDDEN))?;

    if !request.has_data() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let db_pool = &state.db_pool;
    let mut trans = db_pool.begin().await.map_err(db::DbQueryError::from)?;

    if let Some(pwd) = request.password {
        db::set_password(&mut trans, &player_name, &pwd.hash, &pwd.salt).await?;
    }

    if let Some(is_mojang) = request.is_mojang {
        db::set_is_mojang(&mut trans, &player_name, is_mojang).await?;
    }

    // Renaming should be the last operation
    if let Some(new_player_name) = request.player_name {
        db::set_player_name(&mut trans, &player_name, &new_player_name).await?;
    }

    trans.commit().await.map_err(db::DbLookupError::from)?;

    Ok(StatusCode::OK)
}

pub async fn query_player(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(player_name): Path<String>,
) -> Result<PlayerInfoResponse, StatusCode> {
    validate_api_token(&state, &headers).or(Err(StatusCode::FORBIDDEN))?;

    let record = db::get_player_record_by_name(&state.db_pool, &player_name).await?;

    Ok(PlayerInfoResponse {
        player_id: record.player_id,
        is_mojang: record.is_mojang,
    })
}

fn validate_api_token(state: &AppState, headers: &HeaderMap) -> Result<(), ()> {
    let access_token = util::get_access_token_from_header(headers).ok_or(())?;
    if access_token == state.internal_api_access_token {
        Ok(())
    } else {
        Err(())
    }
}

pub async fn set_skin(
    State(state): State<AppState>,
    ReJson(request): ReJson<SetSkinRequest>,
) -> Result<StatusResponse, ErrorResponse> {
    if request.data.len() > assets::MAX_SKIN_BASE64_LEN {
        return Err(ErrorResponse::bad_request_with_msg("Skin file is too big"));
    }

    let credentials = db::get_player_auth_record(&state.db_pool, &request.username).await?;
    if !util::check_password(&request.password, &credentials) {
        return Err(ErrorResponse::unauthorized());
    }

    let skin_name = credentials.player_id;
    assets::save_skin(&state.assets_cfg, request.data.as_bytes(), &skin_name).await?;

    let is_slim = request.skin_model.map_or(false, |model| model == "slim");
    db::set_skin_model(&state.db_pool, &request.username, is_slim).await?;

    Ok(StatusResponse::accepted())
}

pub async fn login(
    State(state): State<AppState>,
    ReJson(request): ReJson<LoginRequest>,
) -> Result<LoginResponse, ErrorResponse> {
    if !request.validate() {
        return Err(ErrorResponse::bad_request());
    }

    let player = db::get_player_auth_record(&state.db_pool, &request.username).await?;
    if player.is_mojang {
        const MESSAGE: &str =
            "Disable Mojang authentication in your profile, or use the official launcher";
        return Err(ErrorResponse::forbidden_with_msg(MESSAGE));
    }

    if !util::check_password(&request.password, &player) {
        return Err(ErrorResponse::unauthorized());
    }

    let access_token = Uuid::new_v4().hyphenated().to_string();
    db::set_access_token(&state.db_pool, &request.username, &access_token).await?;

    if state.collect_stats {
        let _ = db::collect_stats(&state.db_pool, (&request).into()).await;
    }

    Ok(LoginResponse {
        player_id: player.player_id,
        access_token,
    })
}

pub async fn join(
    State(state): State<AppState>,
    ReJson(request): ReJson<JoinRequest>,
) -> Result<StatusCode, StatusCode> {
    if !request.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let Some(player_id) = util::to_hyphenated_uuid(&request.player_id_compact) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let player = db::get_player_join_record(&state.db_pool, &player_id).await?;
    if player.is_mojang {
        return Err(StatusCode::FORBIDDEN);
    }

    if player.access_token != request.access_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    state
        .pending_joins
        .lock()
        .await
        .insert(player_id, request.server_id);

    Ok(StatusCode::NO_CONTENT)
}

pub async fn has_joined(
    State(state): State<AppState>,
    Query(query): Query<HasJoinedRequestQuery>,
) -> Result<PlayerProfile, StatusCode> {
    if !query.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let player = db::get_player_record_by_name(&state.db_pool, &query.username).await?;
    if player.is_mojang {
        return mojang::has_joined(player, &query.server_id).await;
    }

    {
        let mut pending_joins = state.pending_joins.lock().await;
        let pending_server_id = pending_joins
            .get(&player.player_id)
            .ok_or(StatusCode::NO_CONTENT)?;

        if &query.server_id != pending_server_id {
            return Err(StatusCode::NO_CONTENT);
        }

        pending_joins.remove(&player.player_id);
    }

    let response = player_profile::get(&state, player, true)
        .await
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}

pub async fn profile(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<ProfileRequestQuery>,
) -> Result<PlayerProfile, StatusCode> {
    let Some(player_id) = util::to_hyphenated_uuid(&id) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let player = db::get_player_record_by_id(&state.db_pool, &player_id).await?;
    let unsigned = query.unsigned.unwrap_or(true);
    let response = player_profile::get(&state, player, !unsigned)
        .await
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}

pub async fn player_attributes() -> Response {
    let payload = json!({
        "privileges": {
            "onlineChat":        { "enabled": true  },
            "multiplayerServer": { "enabled": true  },
            "multiplayerRealms": { "enabled": true  },
            "telemetry":         { "enabled": false }
        },
        "profanityFilterPreferences": { "profanityFilterOn": false },
        "banStatus": { "bannedScopes": {} }
    });

    Json(payload).into_response()
}

pub async fn player_certificates(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<PlayerCertificates, StatusCode> {
    let access_token = util::get_access_token_from_header(&headers).ok_or(StatusCode::FORBIDDEN)?;
    let player_id = db::get_player_id_by_access_token(&state.db_pool, access_token).await?;
    let certificates = signing::get_player_certificates(&state.signing, &player_id)
        .await
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(certificates)
}

pub async fn player_report() -> ErrorResponse {
    ErrorResponse::forbidden_with_msg(
        "You don't have reporting permissions. This incident will be reported",
    )
}

pub async fn public_keys(State(state): State<AppState>) -> Response {
    let public_key = &state.signing.public_key_base64;
    let payload = json!({
        "profilePropertyKeys":   [ { "publicKey": public_key } ],
        "playerCertificateKeys": [ { "publicKey": public_key } ]
    });
    Json(payload).into_response()
}

pub async fn block_list() -> Response {
    let payload = json!({ "blockedProfiles": [] });
    Json(payload).into_response()
}

mod error_to_error_response {
    use crate::assets::SkinSaveError;
    use crate::db::{DbLookupError, DbQueryError};
    use crate::payload::ErrorResponse;

    impl From<DbQueryError> for ErrorResponse {
        fn from(_: DbQueryError) -> Self {
            ErrorResponse::internal_error()
        }
    }

    impl From<DbLookupError> for ErrorResponse {
        fn from(error: DbLookupError) -> Self {
            match error {
                DbLookupError::NotFound => ErrorResponse::unauthorized(),
                DbLookupError::QueryError => ErrorResponse::internal_error(),
            }
        }
    }

    impl From<SkinSaveError> for ErrorResponse {
        fn from(error: SkinSaveError) -> Self {
            match error {
                SkinSaveError::IoError => ErrorResponse::internal_error(),
                SkinSaveError::InvalidImage => ErrorResponse::bad_request(),
                SkinSaveError::InvalidImageSize => {
                    ErrorResponse::bad_request_with_msg("Invalid skin image size")
                }
            }
        }
    }
}

mod error_to_status_code {
    use crate::db::{DbLookupError, DbQueryError};
    use axum::http::StatusCode;

    impl From<DbQueryError> for StatusCode {
        fn from(_: DbQueryError) -> Self {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    impl From<DbLookupError> for StatusCode {
        fn from(error: DbLookupError) -> Self {
            match error {
                DbLookupError::NotFound => StatusCode::NOT_FOUND,
                DbLookupError::QueryError => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
}
