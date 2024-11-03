mod analytics;

pub use analytics::*;
use analytics_event::EventType;

impl From<UserLoginEvent> for EventType {
    fn from(event: UserLoginEvent) -> Self {
        EventType::UserLogin(event)
    }
}

impl From<UserLogoutEvent> for EventType {
    fn from(event: UserLogoutEvent) -> Self {
        EventType::UserLogout(event)
    }
}

impl From<AppStartEvent> for EventType {
    fn from(event: AppStartEvent) -> Self {
        EventType::AppStart(event)
    }
}

impl From<AppExitEvent> for EventType {
    fn from(event: AppExitEvent) -> Self {
        EventType::AppExit(event)
    }
}

impl From<MessageSentEvent> for EventType {
    fn from(event: MessageSentEvent) -> Self {
        EventType::MessageSent(event)
    }
}

impl From<NavigationEvent> for EventType {
    fn from(event: NavigationEvent) -> Self {
        EventType::Navigation(event)
    }
}
