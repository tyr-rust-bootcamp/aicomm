use crate::{AppError, AppState};
use axum::http::request::Parts;
use chat_core::{
    pb::{analytics_event::EventType, *},
    User,
};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;

const SESSION_TIMEOUT: i64 = 10 * 60 * 1000; // 10 minutes

#[derive(Debug, Default, Clone, Row, Serialize, Deserialize)]
pub struct AnalyticsEventRow {
    // EventContext fields
    pub client_id: String,
    pub session_id: String,
    pub duration: u32,
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
    pub event_type: String,
    // AppExitEvent fields
    pub exit_code: Option<String>,
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

trait EventConsume {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError>;
}

impl AnalyticsEventRow {
    pub fn update_with_server_info(&mut self, parts: &Parts, geo: Option<GeoLocation>) {
        // get user info from extension
        if let Some(user) = parts.extensions.get::<User>() {
            self.user_id = Some(user.id.to_string());
        } else {
            self.user_id = None;
        }

        // use server geo info
        if let Some(geo) = geo {
            self.geo_country = Some(geo.country);
            self.geo_region = Some(geo.region);
            self.geo_city = Some(geo.city);
        } else {
            self.geo_country = None;
            self.geo_region = None;
            self.geo_city = None;
        }

        // override server_ts with current time
        self.server_ts = chrono::Utc::now().timestamp_millis();
    }

    pub fn set_session_id(&mut self, state: &AppState) {
        if let Some(mut v) = state.sessions.get_mut(&self.client_id) {
            let (session_id, last_server_ts) = v.value_mut();
            let mut duration = self.server_ts - *last_server_ts;
            if duration < 0 {
                warn!("Session {} duration is negative, reset to 0", session_id);
                duration = 0;
            }
            if duration < SESSION_TIMEOUT {
                self.session_id = session_id.clone();
                self.duration = duration as u32;
                *last_server_ts = self.server_ts;
            } else {
                let new_session_id = Uuid::now_v7().to_string();
                self.session_id = new_session_id.clone();
                self.duration = 0;
                info!(
                    "Session {} expired, start a new session: {}",
                    session_id, new_session_id
                );
                *last_server_ts = self.server_ts;
                *session_id = new_session_id;
            }
        } else {
            let session_id = Uuid::now_v7().to_string();
            self.session_id = session_id.clone();
            self.duration = 0;
            info!("No client id found, start a new session: {}", session_id);
            state
                .sessions
                .insert(self.client_id.clone(), (session_id, self.server_ts));
        }
    }
}

impl TryFrom<AnalyticsEvent> for AnalyticsEventRow {
    type Error = crate::AppError;

    fn try_from(event: AnalyticsEvent) -> Result<Self, Self::Error> {
        let mut ret = Self::default();
        match event.context {
            Some(context) => context.consume(&mut ret)?,
            None => return Err(AppError::MissingEventContext),
        }

        match event.event_type {
            Some(event) => event.consume(&mut ret)?,
            None => return Err(AppError::MissingEventData),
        }
        Ok(ret)
    }
}

impl EventConsume for EventContext {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.client_id = self.client_id;
        row.app_version = self.app_version;

        if let Some(system) = self.system {
            row.system_os = system.os;
            row.system_arch = system.arch;
            row.system_locale = system.locale;
            row.system_timezone = system.timezone;
        } else {
            return Err(AppError::MissingSystemInfo);
        }

        if !self.user_id.is_empty() {
            row.user_id = Some(self.user_id);
        }

        if !self.ip.is_empty() {
            row.ip = Some(self.ip);
        }
        if !self.user_agent.is_empty() {
            row.user_agent = Some(self.user_agent);
        }

        if let Some(geo) = self.geo {
            row.geo_country = Some(geo.country);
            row.geo_region = Some(geo.region);
            row.geo_city = Some(geo.city);
        }

        row.client_ts = self.client_ts;
        row.server_ts = self.server_ts;
        Ok(())
    }
}

impl EventConsume for EventType {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        match self {
            EventType::AppExit(event) => event.consume(row),
            EventType::AppStart(event) => event.consume(row),
            EventType::UserLogin(event) => event.consume(row),
            EventType::UserLogout(event) => event.consume(row),
            EventType::UserRegister(event) => event.consume(row),
            EventType::ChatCreated(event) => event.consume(row),
            EventType::MessageSent(event) => event.consume(row),
            EventType::ChatJoined(event) => event.consume(row),
            EventType::ChatLeft(event) => event.consume(row),
            EventType::Navigation(event) => event.consume(row),
        }
    }
}

impl EventConsume for AppStartEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "app_start".to_string();
        Ok(())
    }
}

impl EventConsume for AppExitEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "app_exit".to_string();
        row.exit_code = Some(self.exit_code().as_str_name().to_string());
        Ok(())
    }
}

impl EventConsume for UserLoginEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "user_login".to_string();
        row.login_email = Some(self.email);
        Ok(())
    }
}

impl EventConsume for UserLogoutEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "user_logout".to_string();
        row.logout_email = Some(self.email);
        Ok(())
    }
}

impl EventConsume for UserRegisterEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "user_register".to_string();
        row.register_email = Some(self.email);
        row.register_workspace_id = Some(self.workspace_id);
        Ok(())
    }
}

impl EventConsume for ChatCreatedEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "chat_created".to_string();
        row.chat_created_workspace_id = Some(self.workspace_id);
        Ok(())
    }
}

impl EventConsume for MessageSentEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "message_sent".to_string();
        row.message_chat_id = Some(self.chat_id);
        row.message_type = Some(self.r#type);
        row.message_size = Some(self.size);
        Ok(())
    }
}

impl EventConsume for ChatJoinedEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "chat_joined".to_string();
        row.chat_joined_id = Some(self.chat_id);
        Ok(())
    }
}

impl EventConsume for ChatLeftEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "chat_left".to_string();
        row.chat_left_id = Some(self.chat_id);
        Ok(())
    }
}

impl EventConsume for NavigationEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = "navigation".to_string();
        row.navigation_from = Some(self.from);
        row.navigation_to = Some(self.to);
        Ok(())
    }
}
