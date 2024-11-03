mod config;
mod error;
mod events;
mod extractors;
mod handlers;
mod openapi;

pub use config::*;
use dashmap::DashMap;
pub use error::*;
pub use events::*;

use anyhow::Context;
use chat_core::{
    middlewares::{extract_user, set_layer, TokenVerify},
    DecodingKey, User,
};
use clickhouse::Client;
use handlers::create_event_handler;
use openapi::OpenApiRouter as _;
use std::{fmt, ops::Deref, sync::Arc};
use tokio::fs;
use tower_http::cors::{self, CorsLayer};

use axum::{http::Method, middleware::from_fn_with_state, routing::post, Router};

pub use config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
pub struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) client: Client,
    pub(crate) sessions: Arc<DashMap<String, (String, i64)>>,
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);
    let api = Router::new()
        .route("/event", post(create_event_handler))
        .layer(from_fn_with_state(state.clone(), extract_user::<AppState>))
        // routes doesn't need token verification
        .layer(cors);

    let app = Router::new().openapi().nest("/api", api).with_state(state);

    Ok(set_layer(app))
}

// 当我调用 state.config => state.inner.config
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.dk.verify(token)?)
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir)
            .await
            .context("create base_dir failed")?;
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let mut client = Client::default()
            .with_url(&config.server.db_url)
            .with_database(&config.server.db_name);
        if let Some(user) = config.server.db_user.as_ref() {
            client = client.with_user(user);
        }
        if let Some(password) = config.server.db_password.as_ref() {
            client = client.with_password(password);
        }
        // TODO: load sessions from db
        let sessions = Arc::new(DashMap::new());
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                dk,
                client,
                sessions,
            }),
        })
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}
