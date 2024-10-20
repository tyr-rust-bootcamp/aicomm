use analytics_server::AnalyticsEventRow;
use anyhow::Result;
use clickhouse::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_user("default")
        .with_database("analytics");

    let mut cursor = client
        .query("SELECT * FROM analytics_events limit 1")
        .fetch::<AnalyticsEventRow>()?;

    while let Some(row) = cursor.next().await? {
        println!("{:?}", row);
    }
    Ok(())
}
