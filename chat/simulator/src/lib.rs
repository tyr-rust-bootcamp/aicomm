mod convert;
mod dummy;

use chrono::{DateTime, Utc};
use dummy::*;
use fake::{
    faker::address::en::{CityName, CountryName},
    faker::internet::en::{IPv4, SafeEmail, UserAgent},
    uuid::UUIDv4,
    Dummy,
};

pub struct SimSession {
    pub user: SimUser,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub events: Vec<SimEventType>,
}

#[derive(Debug, Clone, Dummy)]
pub struct SimUser {
    #[dummy(faker = "UUIDv4")]
    pub client_id: String,
    #[dummy(faker = "AppVersion")]
    pub app_version: String,
    #[dummy(faker = "SystemOs")]
    pub system_os: String,
    #[dummy(faker = "SystemArch")]
    pub system_arch: String,
    #[dummy(faker = "SystemLocale")]
    pub system_locale: String,
    #[dummy(faker = "SystemTimezone")]
    pub system_timezone: String,
    #[dummy(faker = "UUIDv4")]
    pub user_id: String,
    // we should use maxminddb to get the real country, region and city: https://github.com/oschwald/maxminddb-rust
    #[dummy(faker = "IPv4()")]
    pub ip: String,
    #[dummy(faker = "UserAgent()")]
    pub user_agent: String,
    #[dummy(faker = "CountryName()")]
    pub geo_country: String,
    #[dummy(faker = "RegionName")]
    pub geo_region: String,
    #[dummy(faker = "CityName()")]
    pub geo_city: String,
}

#[derive(Debug, Clone)]
pub struct SimEvent {
    pub user: SimUser,
    pub event_type: SimEventType,
}

#[derive(Debug, Clone, Dummy)]
pub enum SimEventType {
    Login(LoginData),
    Navigation(NavigationData),
    Message(MessageData),
}

#[derive(Debug, Clone, Dummy)]
pub struct LoginData {
    #[dummy(faker = "SafeEmail()")]
    pub email: String,
}

#[derive(Debug, Clone, Dummy)]
pub struct NavigationData {
    #[dummy(faker = "1..=1000")]
    pub from: u32,
    #[dummy(faker = "1..=1000")]
    pub to: u32,
}

#[derive(Debug, Clone, Dummy)]
pub struct MessageData {
    #[dummy(faker = "UUIDv4")]
    pub chat_id: String,
    #[dummy(faker = "MessageType")]
    pub r#type: String,
    #[dummy(faker = "1..=1000")]
    pub size: u32,
    #[dummy(faker = "0..=10")]
    pub total_files: u32,
}
