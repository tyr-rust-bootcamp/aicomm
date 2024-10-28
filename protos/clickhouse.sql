CREATE TABLE analytics.analytics_events(
    -- EventContext fields
    client_id String,
    session_id String,
    duration UInt32,
    app_version String,
    system_os String,
    system_arch String,
    system_locale String,
    system_timezone String,
    user_id Nullable(String),
    ip Nullable(String),
    user_agent Nullable(String),
    geo_country Nullable(String),
    geo_region Nullable(String),
    geo_city Nullable(String),
    client_ts DateTime64(3),
    server_ts DateTime64(3),
    -- Common fields
    event_type String,
    -- AppExitEvent fields
    exit_code Nullable(String),
    -- UserLoginEvent
    login_email Nullable(String),
    -- UserLogoutEvent
    logout_email Nullable(String),
    -- UserRegisterEvent
    register_email Nullable(String),
    register_workspace_id Nullable(String),
    -- ChatCreatedEvent
    chat_created_workspace_id Nullable(String),
    -- MessageSentEvent
    message_chat_id Nullable(String),
    message_type Nullable(String),
    message_size Nullable(Int32),
    message_total_files Nullable(Int32),
    -- ChatJoinedEvent
    chat_joined_id Nullable(String),
    -- ChatLeftEvent
    chat_left_id Nullable(String),
    -- NavigationEvent
    navigation_from Nullable(String),
    navigation_to Nullable(String)) ENGINE = MergeTree()
ORDER BY
    (
        event_type,
        session_id,
        client_id,
        server_ts
);

-- Create aggregated sessions table
CREATE TABLE analytics.sessions(
    date date,
    client_id String,
    session_id String,
    app_version String,
    system_os String,
    system_arch String,
    system_locale String,
    system_timezone String,
    user_id Nullable(String),
    ip Nullable(String),
    user_agent Nullable(String),
    geo_country Nullable(String),
    geo_region Nullable(String),
    geo_city Nullable(String),
    session_start SimpleAggregateFunction(min, DateTime64(3)),
    session_end SimpleAggregateFunction(max, DateTime64(3)),
    session_length SimpleAggregateFunction(sum, UInt64),
    total_events UInt32) ENGINE = SummingMergeTree()
ORDER BY
    (
        date,
        client_id,
        session_id
);

-- Create materialized view to aggregate sessions
CREATE MATERIALIZED VIEW analytics.sessions_mv TO analytics.sessions AS
SELECT
    toDate(server_ts) AS date,
    client_id,
    session_id,
    any(app_version) AS app_version,
    any(system_os) AS system_os,
    any(system_arch) AS system_arch,
    any(system_locale) AS system_locale,
    any(system_timezone) AS system_timezone,
    any(user_id) AS user_id,
    any(ip) AS ip,
    any(user_agent) AS user_agent,
    any(geo_country) AS geo_country,
    any(geo_region) AS geo_region,
    any(geo_city) AS geo_city,
    min(server_ts) AS session_start,
    max(server_ts) AS session_end,
    sum(duration) / 1000 AS session_length,
    count(1) AS total_events
FROM
    analytics.analytics_events
GROUP BY
    date,
    client_id,
    session_id;

-- populate sessions table
-- INSERT INTO analytics.sessions...;
-- query sessions table
SELECT
    date,
    client_id,
    session_id,
    session_start,
    session_end,
    session_length,
    total_events
FROM
    analytics.sessions FINAL;

CREATE TABLE analytics.daily_sessions(
    date date,
    client_id String,
    total_session_length SimpleAggregateFunction(sum, UInt64),
    total_session_events SimpleAggregateFunction(sum, UInt64),
    unique_users AggregateFunction(uniq, Nullable(String))) ENGINE = SummingMergeTree()
ORDER BY
    (
        date,
        client_id
);

CREATE MATERIALIZED VIEW analytics.daily_sessions_mv TO analytics.daily_sessions AS
SELECT
    date,
    client_id,
    sum(session_length) AS total_session_length,
    sum(total_events) AS total_session_events,
    uniqState(user_id) AS unique_users
FROM
    analytics.sessions
GROUP BY
    date,
    client_id;

SELECT
    date,
    client_id,
    sum(total_session_length) AS total_session_length,
    sum(total_session_events) AS total_session_events,
    uniqMerge(unique_users) AS unique_users
FROM
    analytics.daily_sessions
GROUP BY
    date,
    client_id;

-- Insert sample data for AppStartEvent
INSERT INTO analytics.analytics_events(client_id, session_id, app_version, system_os, system_arch, system_locale, system_timezone, client_ts, server_ts, event_type)
    VALUES ('client_001', 'session_001', '1.0.0', 'macOS', 'x86_64', 'en-US', 'America/New_York', now(), now(), 'AppStart');

-- Insert sample data for UserLoginEvent
INSERT INTO analytics.analytics_events(client_id, session_id, app_version, system_os, system_arch, system_locale, system_timezone, user_id, ip, client_ts, server_ts, event_type, login_email)
    VALUES ('client_002', 'session_002', '1.0.1', 'Windows', 'x86_64', 'en-GB', 'Europe/London', 'user_123', '192.168.1.1', now(), now(), 'UserLogin', 'user@example.com');

-- Insert sample data for MessageSentEvent
INSERT INTO analytics.analytics_events(client_id, session_id, app_version, system_os, system_arch, system_locale, system_timezone, user_id, client_ts, server_ts, event_type, message_chat_id, message_type, message_size, message_total_files)
    VALUES ('client_003', 'session_003', '1.0.2', 'Linux', 'aarch64', 'fr-FR', 'Europe/Paris', 'user_456', now(), now(), 'MessageSent', 'chat_789', 'text', 100, 0);

-- Insert sample data for AppExitEvent
INSERT INTO analytics.analytics_events(client_id, session_id, app_version, system_os, system_arch, system_locale, system_timezone, client_ts, server_ts, event_type, exit_code)
    VALUES ('client_004', 'session_004', '1.0.3', 'iOS', 'arm64', 'ja-JP', 'Asia/Tokyo', now(), now(), 'AppExit', 'SUCCESS');

-- Insert sample data for NavigationEvent
INSERT INTO analytics.analytics_events(client_id, session_id, app_version, system_os, system_arch, system_locale, system_timezone, user_id, client_ts, server_ts, event_type, navigation_from, navigation_to)
    VALUES ('client_005', 'session_005', '1.0.4', 'Android', 'arm64-v8a', 'es-ES', 'Europe/Madrid', 'user_789', now(), now(), 'Navigation', '/home', '/chat');

