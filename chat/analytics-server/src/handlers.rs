use crate::{
    extractors::{Geo, Protobuf},
    pb::AnalyticsEvent,
    AnalyticsEventRow, AppError, AppState,
};
use axum::{
    extract::State,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use chat_core::User;
use tracing::info;

/// Update the agent by id.
#[utoipa::path(
    post,
    path = "/api/event",
    responses(
        (status = 201, description = "Event created"),
        (status = 400, description = "Invalid event", body = ErrorOutput),
        (status = 500, description = "Internal server error", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_event_handler(
    parts: Parts,
    State(state): State<AppState>,
    Geo(geo): Geo,
    Protobuf(event): Protobuf<AnalyticsEvent>,
) -> Result<impl IntoResponse, AppError> {
    let mut row = AnalyticsEventRow::try_from(event)?;

    // get user info from extension
    if let Some(user) = parts.extensions.get::<User>() {
        row.user_id = Some(user.id.to_string());
    } else {
        row.user_id = None;
    }

    // use server geo info
    if let Some(geo) = geo {
        row.geo_country = Some(geo.country);
        row.geo_region = Some(geo.region);
        row.geo_city = Some(geo.city);
    } else {
        row.geo_country = None;
        row.geo_region = None;
        row.geo_city = None;
    }

    // override server_ts with current time
    row.server_ts = chrono::Utc::now().timestamp_millis();

    let data = serde_json::to_string_pretty(&row).unwrap();
    info!("event: {}", data);

    let mut insert = state.client.insert("analytics_events")?;
    insert.write(&row).await?;
    insert.end().await?;
    Ok(StatusCode::CREATED)
}
