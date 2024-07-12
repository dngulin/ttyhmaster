use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub main: MainConfig,
    pub internal_api: InternalApiConfig,
    pub assets: AssetsConfig,
    pub signing: SigningConfig,
}

#[derive(Deserialize)]
pub struct MainConfig {
    pub address: String,
    pub db_url: String,
    pub collect_stats: bool,
    pub debug: bool,
}

#[derive(Deserialize)]
pub struct InternalApiConfig {
    pub access_token: String,
}

#[derive(Deserialize, Clone)]
pub struct AssetsConfig {
    pub root_dir: String,
    pub root_url: String,
    pub default_skin_path: String,
    pub default_skin_slim: bool,
}

#[derive(Deserialize)]
pub struct SigningConfig {
    pub players_keys_dir: String,
    pub server_key_pem: String,
}
