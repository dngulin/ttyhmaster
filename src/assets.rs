use crate::config::AssetsConfig;
use crate::payload::{TextureInfo, TexturesInfo};
use base64::Engine;
use imagesize::ImageType;
use std::path::PathBuf;
use tokio::join;

use crate::util;
use base64::engine::general_purpose::STANDARD as BASE64;

const MAX_SKIN_FILE_SIZE: usize = 1024 * 16;
pub const MAX_SKIN_BASE64_LEN: usize = MAX_SKIN_FILE_SIZE * 133 / 100;

pub const SKINS_DIR: &str = "skins";
pub const CAPES_DIR: &str = "capes";

pub enum SkinSaveError {
    IoError,
    InvalidImage,
    InvalidImageSize,
}

pub async fn save_skin(
    cfg: &AssetsConfig,
    skin_base64: &[u8],
    skin_name: &str,
) -> Result<(), SkinSaveError> {
    let bytes = BASE64.decode(skin_base64).map_err(|error| {
        tracing::error!("Decode skin: {error}");
        SkinSaveError::InvalidImage
    })?;

    let img_type = imagesize::image_type(&bytes).map_err(|_| SkinSaveError::InvalidImage)?;
    if img_type != ImageType::Png {
        return Err(SkinSaveError::InvalidImage);
    }

    let size = imagesize::blob_size(&bytes).map_err(|_| SkinSaveError::InvalidImage)?;
    if size.width != 64 || size.height != 64 {
        return Err(SkinSaveError::InvalidImageSize);
    }

    let path: PathBuf = [&cfg.root_dir, SKINS_DIR, skin_name].iter().collect();
    tokio::fs::write(path, &bytes).await.map_err(|error| {
        tracing::error!("Save skin: {error}");
        SkinSaveError::IoError
    })?;

    Ok(())
}

pub async fn get_textures_info(
    cfg: &AssetsConfig,
    player_id: &str,
    is_slim_model: bool,
) -> TexturesInfo {
    let skin_path: PathBuf = [&cfg.root_dir, SKINS_DIR, player_id].iter().collect();
    let cape_path: PathBuf = [&cfg.root_dir, CAPES_DIR, player_id].iter().collect();
    let (skin_exist, cape_exist) =
        join!(util::file_exists(&skin_path), util::file_exists(&cape_path));

    let skin = skin_exist.then(|| get_skin_tex_info(&cfg.root_url, player_id, is_slim_model));
    let cape = cape_exist.then(|| get_cape_tex_info(&cfg.root_url, player_id));

    // Fallback to default skin
    let skin = skin.or_else(|| {
        let url = format!("{}/{}", &cfg.root_url, &cfg.default_skin_path);
        Some(TextureInfo::skin(url, cfg.default_skin_slim))
    });

    TexturesInfo { skin, cape }
}

fn get_skin_tex_info(root_url: &str, skin_name: &str, is_slim: bool) -> TextureInfo {
    let skin_url = format!("{root_url}/{SKINS_DIR}/{skin_name}");
    TextureInfo::skin(skin_url, is_slim)
}

fn get_cape_tex_info(root_url: &str, cape_name: &str) -> TextureInfo {
    let cape_url = format!("{root_url}/{CAPES_DIR}/{cape_name}");
    TextureInfo::cape(cape_url)
}
