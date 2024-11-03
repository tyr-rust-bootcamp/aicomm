mod utils;

pub mod middlewares;
pub mod pb;

pub use utils::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;
use utoipa::ToSchema;

#[allow(async_fn_in_trait)]
pub trait Agent {
    async fn process(&self, msg: &str, ctx: &AgentContext) -> Result<AgentDecision, AgentError>;
}

#[derive(Debug, Default, Clone)]
pub struct AgentContext {}

#[derive(Debug, Clone)]
pub enum AgentDecision {
    Modify(String),
    Reply(String),
    Delete,
    None,
}

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("{0}")]
    AnyError(#[from] anyhow::Error),
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    #[sqlx(default)]
    pub ws_name: String,
    pub fullname: String,
    pub email: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password_hash: Option<String>,
    #[sqlx(default)]
    pub is_bot: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChatUser {
    pub id: i64,
    pub fullname: String,
    pub email: String,
}

#[derive(
    Debug, Clone, Default, ToSchema, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type,
)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase"))]
pub enum ChatType {
    #[serde(alias = "single", alias = "Single")]
    #[default]
    Single,
    #[serde(alias = "group", alias = "Group")]
    Group,
    #[serde(alias = "private_channel", alias = "privateChannel")]
    PrivateChannel,
    #[serde(alias = "public_channel", alias = "publicChannel")]
    PublicChannel,
}

#[derive(Debug, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Chat {
    pub id: i64,
    #[serde(alias = "wsId")]
    pub ws_id: i64,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub members: Vec<i64>,
    pub agents: Vec<i64>,
    #[serde(alias = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Message {
    pub id: i64,
    #[serde(alias = "chatId")]
    pub chat_id: i64,
    #[serde(alias = "senderId")]
    pub sender_id: i64,
    pub content: String,
    pub modified_content: Option<String>,
    pub files: Vec<String>,
    #[serde(alias = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(
    Debug, Clone, Default, ToSchema, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type,
)]
#[sqlx(type_name = "agent_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase"))]
pub enum AgentType {
    #[serde(alias = "proxy", alias = "Proxy")]
    #[default]
    Proxy,
    #[serde(alias = "reply", alias = "Reply")]
    Reply,
    #[serde(alias = "tap", alias = "Tap")]
    Tap,
}

#[derive(
    Debug, Clone, Default, ToSchema, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type,
)]
#[sqlx(type_name = "adapter_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase"))]
pub enum AdapterType {
    #[serde(alias = "openai", alias = "Openai")]
    #[default]
    Openai,
    #[serde(alias = "ollama", alias = "Ollama")]
    Ollama,
    #[serde(alias = "test", alias = "Test")]
    Test,
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ChatAgent {
    pub id: i64,
    #[serde(alias = "chatId")]
    pub chat_id: i64,
    pub name: String,
    pub r#type: AgentType,
    pub adapter: AdapterType,
    pub model: String,
    pub prompt: String,
    pub args: sqlx::types::Json<serde_json::Value>, // TODO: change to custom type
    #[serde(alias = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(alias = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: i64, fullname: &str, email: &str) -> Self {
        Self {
            id,
            ws_id: 0,
            ws_name: "".to_string(),
            fullname: fullname.to_string(),
            email: email.to_string(),
            password_hash: None,
            is_bot: false,
            created_at: chrono::Utc::now(),
        }
    }
}
