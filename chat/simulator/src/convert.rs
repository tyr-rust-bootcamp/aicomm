use std::{cmp::Reverse, ops::Range};

use crate::{LoginData, MessageData, NavigationData, SimEvent, SimEventType, SimSession, SimUser};
use analytics_server::AnalyticsEventRow;
use anyhow::Result;
use chat_core::pb::{
    AnalyticsEvent, EventContext, GeoLocation, MessageSentEvent, NavigationEvent, SystemInfo,
    UserLoginEvent,
};
use chrono::{DateTime, Duration, Utc};
use fake::{Fake, Faker};
use rand::{distributions::Uniform, Rng as _};
use uuid::{ContextV7, Timestamp, Uuid};

const SERVER_TS_MILLIS: Range<i64> = 1..100;
const SESSION_START_OFFSET_DAYS: Range<i64> = 1..540;
const SESSION_LENGTH_MINUTES: Range<i64> = 10..120;

impl From<LoginData> for UserLoginEvent {
    fn from(data: LoginData) -> Self {
        UserLoginEvent { email: data.email }
    }
}

impl From<NavigationData> for NavigationEvent {
    fn from(data: NavigationData) -> Self {
        NavigationEvent {
            from: format!("/chats/{}", data.from),
            to: format!("/chats/{}", data.to),
        }
    }
}

impl From<MessageData> for MessageSentEvent {
    fn from(data: MessageData) -> Self {
        MessageSentEvent {
            chat_id: data.chat_id,
            r#type: data.r#type,
            size: data.size as i32,
            total_files: data.total_files as i32,
        }
    }
}

impl From<SimUser> for EventContext {
    fn from(data: SimUser) -> Self {
        EventContext {
            client_id: data.client_id,
            app_version: data.app_version,
            system: Some(SystemInfo {
                os: data.system_os,
                arch: data.system_arch,
                locale: data.system_locale,
                timezone: data.system_timezone,
            }),
            user_id: data.user_id,
            ip: data.ip,
            user_agent: data.user_agent,
            geo: Some(GeoLocation {
                country: data.geo_country,
                city: data.geo_city,
                region: data.geo_region,
            }),
            client_ts: 0,
            server_ts: 0,
        }
    }
}

impl From<SimEvent> for AnalyticsEvent {
    fn from(data: SimEvent) -> Self {
        let context = data.user.into();
        let event = match data.event_type {
            SimEventType::Login(login_data) => UserLoginEvent::from(login_data).into(),
            SimEventType::Navigation(navigation_data) => {
                NavigationEvent::from(navigation_data).into()
            }
            SimEventType::Message(message_data) => MessageSentEvent::from(message_data).into(),
        };
        AnalyticsEvent {
            context: Some(context),
            event_type: Some(event),
        }
    }
}

impl SimSession {
    pub fn new(user: &SimUser, start: DateTime<Utc>, end: DateTime<Utc>, events: usize) -> Self {
        let events = (0..events).map(|_| Faker.fake()).collect();
        SimSession {
            user: user.clone(),
            events,
            start,
            end,
        }
    }

    pub fn list(user: &SimUser, sessions: usize, events: usize) -> Vec<Self> {
        let rng = &mut rand::thread_rng();
        let range = Uniform::from(SESSION_START_OFFSET_DAYS);
        let mut rand_days: Vec<i64> = rng.sample_iter(&range).take(sessions).collect();
        rand_days.sort_by_key(|d| Reverse(*d));

        rand_days
            .into_iter()
            .map(|days| {
                let start = Utc::now() - Duration::days(days);
                let end = start + Duration::minutes(rng.gen_range(SESSION_LENGTH_MINUTES));
                Self::new(user, start, end, events)
            })
            .collect()
    }

    pub fn to_analytics_events(self) -> Result<Vec<AnalyticsEventRow>> {
        let context = ContextV7::new();
        let session_id = Uuid::new_v7(Timestamp::from_unix(
            &context,
            self.start.timestamp() as _,
            self.start.timestamp_subsec_nanos() as _,
        ))
        .to_string();
        let start = self.start.timestamp_millis();
        let end = self.end.timestamp_millis();
        let interval = (end - start) / self.events.len() as i64;
        let mut events = Vec::new();
        for (i, event) in self.events.into_iter().enumerate() {
            let sim_event = SimEvent {
                user: self.user.clone(),
                event_type: event,
            };
            let mut row: AnalyticsEventRow = AnalyticsEvent::from(sim_event).try_into()?;
            // update event row with session id and timestamp/duration
            row.session_id = session_id.clone();
            row.client_ts = start + i as i64 * interval;
            row.server_ts = row.client_ts + rand::thread_rng().gen_range(SERVER_TS_MILLIS);
            row.duration = interval as u32;

            events.push(row);
        }
        Ok(events)
    }
}
