mod config;
mod notif;

pub use config::AppConfig;
pub use notif::setup_pg_listener;

pub const VECTOR_SIZE: usize = 1536;
