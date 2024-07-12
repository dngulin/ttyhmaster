use crate::config::SigningConfig;
use crate::payload::{KeyPair, PlayerCertificates};
use crate::util;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use chrono::{Duration, Utc};
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{DecodePrivateKey, Document, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::signature::{SignatureEncoding, Signer};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha1::Sha1;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub struct SigningData {
    pub signing_key: SigningKey<Sha1>,
    pub players_keys_dir: String,
    pub public_key_base64: String,
}

impl SigningData {
    pub async fn new(config: &SigningConfig) -> anyhow::Result<Self> {
        let private_key_bytes = fs::read(&config.server_key_pem).await?;
        let private_key_string = String::from_utf8(private_key_bytes)?;
        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_string)?;

        let public_key = RsaPublicKey::from(&private_key);
        let public_key_base_64 = BASE64.encode(public_key.to_public_key_der()?.as_bytes());

        let result = Self {
            signing_key: SigningKey::<Sha1>::new(private_key),
            players_keys_dir: config.players_keys_dir.clone(),
            public_key_base64: public_key_base_64,
        };

        Ok(result)
    }
}

pub fn sign_base64(signing: &SigningData, data: &[u8]) -> anyhow::Result<String> {
    let signature = signing.signing_key.try_sign(data).map_err(|error| {
        tracing::error!("Sign data: {error}");
        error
    })?;

    let encoded = BASE64.encode(signature.to_bytes());
    Ok(encoded)
}

pub async fn get_player_certificates(
    signing: &SigningData,
    player_id: &str,
) -> Option<PlayerCertificates> {
    let keypair_path: PathBuf = [&signing.players_keys_dir, player_id].iter().collect();
    let keypair = get_keypair(&keypair_path)
        .await
        .map_err(|error| tracing::error!("Get keypair: {error}"))
        .ok()?;

    let now = Utc::now();
    let expires_at = now + Duration::days(7);
    let refreshed_after = expires_at - Duration::hours(8);

    let timestamp = expires_at.timestamp_millis();
    let signature = sign_public_key_base64(signing, player_id, timestamp, &keypair.public_key)
        .await
        .map_err(|error| tracing::error!("Sign public key: {error}"))
        .ok()?;

    let expires_at = expires_at.format("%+").to_string();
    let refreshed_after = refreshed_after.format("%+").to_string();

    let certificates = PlayerCertificates {
        keypair,
        signature,
        expires_at,
        refreshed_after,
    };
    Some(certificates)
}

async fn get_keypair(path: &Path) -> anyhow::Result<KeyPair> {
    if util::file_exists(path).await {
        let bytes = fs::read(&path).await?;
        let keypair: KeyPair = serde_json::from_slice(&bytes)?;
        return Ok(keypair);
    }

    let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), 2048)?;
    let public_key = RsaPublicKey::from(&private_key);

    let private_key = private_key
        .to_pkcs8_der()?
        .to_pem("RSA PRIVATE KEY", LineEnding::LF)?
        .to_string();

    let public_key = public_key
        .to_public_key_der()?
        .to_pem("RSA PUBLIC KEY", LineEnding::LF)?;

    let keypair = KeyPair {
        private_key,
        public_key,
    };

    let json = serde_json::to_string(&keypair)?;
    fs::write(path, &json).await?;

    Ok(keypair)
}

async fn sign_public_key_base64(
    signing: &SigningData,
    player_id: &str,
    timestamp: i64,
    public_key: &str,
) -> anyhow::Result<String> {
    let (msb, lsb) = Uuid::try_parse(player_id)?.as_u64_pair();

    let mut data: Vec<u8> = Vec::new();

    data.write_all(&msb.to_be_bytes()).await?;
    data.write_all(&lsb.to_be_bytes()).await?;
    data.write_all(&timestamp.to_be_bytes()).await?;

    // Encode key as PKIX/X.509 SubjectPublicKeyInfo
    let (_, pkix_spki) = Document::from_pem(public_key)?;
    data.write_all(pkix_spki.as_bytes()).await?;

    sign_base64(signing, &data)
}
