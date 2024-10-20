use crate::{extractors::Protobuf, pb::AnalyticsEvent, AppError, AppState};
use axum::{
    extract::State,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use chat_core::User;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Row, Serialize, Deserialize)]
pub struct AnalyticsEventRow {
    // EventContext fields
    pub client_id: String,
    pub app_version: String,
    pub system_os: String,
    pub system_arch: String,
    pub system_locale: String,
    pub system_timezone: String,
    pub user_id: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub geo_country: Option<String>,
    pub geo_region: Option<String>,
    pub geo_city: Option<String>,
    pub client_ts: i64,
    pub server_ts: i64,
    // Common fields
    pub event_type: EventTypeRow,
    // AppExitEvent fields
    pub exit_code: Option<ExitCodeRow>,
    // UserLoginEvent
    pub login_email: Option<String>,
    // UserLogoutEvent
    pub logout_email: Option<String>,
    // UserRegisterEvent
    pub register_email: Option<String>,
    pub register_workspace_id: Option<String>,
    // ChatCreatedEvent
    pub chat_created_workspace_id: Option<String>,
    // MessageSentEvent
    pub message_chat_id: Option<String>,
    pub message_type: Option<String>,
    pub message_size: Option<i32>,
    pub message_total_files: Option<i32>,
    // ChatJoinedEvent
    pub chat_joined_id: Option<String>,
    // ChatLeftEvent
    pub chat_left_id: Option<String>,
    // NavigationEvent
    pub navigation_from: Option<String>,
    pub navigation_to: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventTypeRow {
    AppStart,
    AppExit,
    UserLogin,
    UserLogout,
    UserRegister,
    ChatCreated,
    MessageSent,
    ChatJoined,
    ChatLeft,
    Navigation,
    #[default]
    Unspecified,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitCodeRow {
    #[default]
    Unspecified,
    Success,
    Failure,
}

/// Update the agent by id.
#[utoipa::path(
    patch,
    path = "/api/event",
    responses(
        (status = 201, description = "Event created"),
        (status = 400, description = "Invalid event", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_event_handler(
    parts: Parts,
    State(state): State<AppState>,
    Protobuf(event): Protobuf<AnalyticsEvent>,
) -> Result<impl IntoResponse, AppError> {
    let mut row = AnalyticsEventRow::try_from(event)?;
    // get user info from extension
    if let Some(user) = parts.extensions.get::<User>() {
        row.user_id = Some(user.id.to_string());
    } else {
        row.user_id = None;
    }

    let mut insert = state.client.insert("analytics")?;
    insert.write(&row).await?;
    insert.end().await?;
    Ok(StatusCode::CREATED)
}
