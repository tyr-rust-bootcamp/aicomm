use crate::{
    handler::{AnalyticsEventRow, EventTypeRow, ExitCodeRow},
    pb::{analytics_event::EventType, app_exit_event::ExitCode, *},
    AppError,
};

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
        row.event_type = EventTypeRow::AppStart;
        Ok(())
    }
}

impl EventConsume for AppExitEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::AppExit;
        row.exit_code = Some(self.exit_code().into());
        Ok(())
    }
}

impl EventConsume for UserLoginEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::UserLogin;
        row.login_email = Some(self.email);
        Ok(())
    }
}

impl EventConsume for UserLogoutEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::UserLogout;
        row.logout_email = Some(self.email);
        Ok(())
    }
}

impl EventConsume for UserRegisterEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::UserRegister;
        row.register_email = Some(self.email);
        row.register_workspace_id = Some(self.workspace_id);
        Ok(())
    }
}

impl EventConsume for ChatCreatedEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::ChatCreated;
        row.chat_created_workspace_id = Some(self.workspace_id);
        Ok(())
    }
}

impl EventConsume for MessageSentEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::MessageSent;
        row.message_chat_id = Some(self.chat_id);
        row.message_type = Some(self.r#type);
        row.message_size = Some(self.size);
        Ok(())
    }
}

impl EventConsume for ChatJoinedEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::ChatJoined;
        row.chat_joined_id = Some(self.chat_id);
        Ok(())
    }
}

impl EventConsume for ChatLeftEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::ChatLeft;
        row.chat_left_id = Some(self.chat_id);
        Ok(())
    }
}

impl EventConsume for NavigationEvent {
    fn consume(self, row: &mut AnalyticsEventRow) -> Result<(), AppError> {
        row.event_type = EventTypeRow::Navigation;
        row.navigation_from = Some(self.from);
        row.navigation_to = Some(self.to);
        Ok(())
    }
}

impl From<ExitCode> for ExitCodeRow {
    fn from(exit_code: ExitCode) -> Self {
        match exit_code {
            ExitCode::Unspecified => ExitCodeRow::Unspecified,
            ExitCode::Success => ExitCodeRow::Success,
            ExitCode::Failure => ExitCodeRow::Failure,
        }
    }
}
