use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PlayerCertificates {
    #[serde(rename = "keyPair")]
    pub keypair: KeyPair,
    #[serde(rename = "publicKeySignatureV2")]
    pub signature: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    #[serde(rename = "refreshedAfter")]
    pub refreshed_after: String,
}

#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl IntoResponse for PlayerCertificates {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
