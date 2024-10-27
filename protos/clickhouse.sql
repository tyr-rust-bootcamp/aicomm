CREATE TABLE analytics.analytics_events(
    -- EventContext fields
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
