use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SetSkinRequest {
    pub username: String,
    pub password: String,
    #[serde(rename = "skinData")]
    pub data: String,
    #[serde(rename = "skinModel")]
    pub skin_model: Option<String>,
}

#[derive(Serialize)]
pub struct TexturesProperty {
    pub timestamp: i64,
    #[serde(rename = "profileId")]
    pub player_id: String,
    #[serde(rename = "profileName")]
    pub player_name: String,
    #[serde(rename = "signatureRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_required: Option<bool>,
    pub textures: TexturesInfo,
}

#[derive(Serialize)]
pub struct TexturesInfo {
    #[serde(rename = "SKIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<TextureInfo>,
    #[serde(rename = "CAPE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cape: Option<TextureInfo>,
}

#[derive(Serialize)]
pub struct TextureInfo {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<TextureMetadata>,
}

impl TextureInfo {
    pub fn skin(url: String, is_slim: bool) -> Self {
        Self {
            url,
            metadata: match is_slim {
                true => Some(TextureMetadata { model: "slim" }),
                false => None,
            },
        }
    }

    pub fn cape(url: String) -> Self {
        Self {
            url,
            metadata: None,
        }
    }
}

#[derive(Serialize)]
pub struct TextureMetadata {
    pub model: &'static str,
}
