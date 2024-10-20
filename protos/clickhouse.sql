CREATE TABLE analytics_events(
    -- EventContext fields
    client_id String,
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
    event_type Enum8('app_start' = 1, 'app_exit' = 2, 'user_login' = 3, 'user_logout' = 4, 'user_register' = 5, 'chat_created' = 6, 'message_sent' = 7, 'chat_joined' = 8, 'chat_left' = 9, 'navigation' = 10),
    -- AppExitEvent fields
    exit_code Nullable(Enum8('EXIT_CODE_UNSPECIFIED' = 0, 'EXIT_CODE_SUCCESS' = 1, 'EXIT_CODE_FAILURE' = 2)),
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
        server_ts,
        event_type,
        client_id
);

