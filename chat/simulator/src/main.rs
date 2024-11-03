use anyhow::Result;
use clickhouse::Client;
use fake::{Fake, Faker};
use simulator::{SimSession, SimUser};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let users = sim_users(1000);

    let sessions: Vec<SimSession> = users
        .iter()
        .flat_map(|user| SimSession::list(user, 1000, 100))
        .collect();

    let rows = sessions
        .into_iter()
        .flat_map(|session| session.to_analytics_events().unwrap());

    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_database("analytics");

    let mut insert = client.insert("analytics_events")?;
    for (i, row) in rows.enumerate() {
        // info!(
        //     "user: {:?}, session: {}, event_type: {}",
        //     row.user_id, row.session_id, row.event_type
        // );
        if i % 1000 == 0 {
            print!(".");
        }
        insert.write(&row).await?;
    }
    insert.end().await?;
    println!("Done!");

    Ok(())
}

fn sim_users(count: usize) -> Vec<SimUser> {
    (0..count).map(|_| Faker.fake::<SimUser>()).collect()
}
