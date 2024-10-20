use crate::{
    pb::{analytics_event::EventType, *},
    AppError,
};
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
