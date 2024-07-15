mod api;
mod assets;
mod config;
mod db;
mod extensions;
mod mojang;
mod payload;
mod player_profile;
mod signing;
mod util;

use crate::config::Config;
use crate::extensions::ReRoute;

use axum::extract::Request;
use axum::http::Uri;
use axum::routing::{get, post};
use axum::{Router, ServiceExt};
use tokio::fs;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_str = fs::read_to_string("config.toml").await?;
    let config = toml::from_str::<Config>(&config_str)?;
    let app_state = state::new_app_state(&config).await?;

    tracer::initialize(config.main.debug);

    let router: Router = Router::new()
        // Ttyh Public API (Launcher)
        .route("/ttyh/set_skin", post(api::set_skin))
        .route("/ttyh/login", post(api::login))
        // Ttyh Internal API
        .route("/ttyh/player", post(api::create_player))
        .route("/ttyh/player/:name", get(api::query_player))
        .route("/ttyh/player/:name", post(api::update_player))
        // Session & Player API
        .route("/session/minecraft/join", post(api::join))
        .route("/session/minecraft/hasJoined", get(api::has_joined))
        .route("/session/minecraft/profile/:id", get(api::profile))
        .route("/player/attributes", get(api::player_attributes))
        .route("/player/certificates", post(api::player_certificates))
        .route("/player/report", post(api::player_report))
        .route("/publickeys", get(api::public_keys))
        .route("/privacy/blocklist", get(api::block_list))
        // Skins & Capes
        .nest_service("/", ServeDir::new(&config.assets.root_dir))
        // Handlers State and Logging Layer
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    // Compatibility layer for the old communication protocol
    // Axum doesn't support routes to `path+query` URIs,
    // so we reroute old-fashion URIs to new ones
    let mappings = vec![
        mapping("/index.php?act=login", "/ttyh/login"),
        mapping("/index.php?act=setskin", "/ttyh/set_skin"),
    ];
    let re_route = ReRoute::wrap(router, mappings);

    let service = ServiceExt::<Request>::into_make_service(re_route);
    let listener = tokio::net::TcpListener::bind(&config.main.address).await?;

    axum::serve(listener, service).await?;

    Ok(())
}

fn mapping(from: &'static str, to: &'static str) -> (Uri, Uri) {
    (Uri::from_static(from), Uri::from_static(to))
}

mod state {
    use crate::config::{AssetsConfig, Config};
    use crate::db;
    use crate::signing::SigningData;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type AppState = Arc<AppStateInner>;

    pub struct AppStateInner {
        pub db_pool: db::DbPool,
        pub internal_api_access_token: String,
        pub collect_stats: bool,
        pub pending_joins: Mutex<HashMap<String, String>>,
        pub assets_cfg: AssetsConfig,
        pub signing: SigningData,
    }

    pub async fn new_app_state(config: &Config) -> anyhow::Result<AppState> {
        let inner = AppStateInner {
            db_pool: db::DbPool::connect(&config.main.db_url).await?,
            internal_api_access_token: config.internal_api.access_token.clone(),
            collect_stats: config.main.collect_stats,
            pending_joins: Mutex::new(HashMap::with_capacity(16)),
            assets_cfg: config.assets.clone(),
            signing: SigningData::new(&config.signing).await?,
        };

        Ok(Arc::new(inner))
    }
}

mod tracer {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::filter::Targets;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    pub fn initialize(debug: bool) {
        let log_level = if debug {
            LevelFilter::DEBUG
        } else {
            LevelFilter::INFO
        };

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().pretty())
            .with(Targets::new().with_default(log_level))
            .init();
    }
}
