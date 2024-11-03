use anyhow::Result;
use bot_server::{AppConfig, VECTOR_SIZE};
use sqlx::postgres::PgPoolOptions;
use swiftide::{
    indexing::{
        self,
        loaders::FileLoader,
        transformers::{ChunkCode, Embed, MetadataQACode},
    },
    integrations,
};
use swiftide_pgvector::PgVector;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let db_url = &config.server.db_url;

    let pool = PgPoolOptions::new().connect(db_url).await?;
    let client = integrations::openai::OpenAI::builder()
        .default_embed_model("text-embedding-3-small")
        .default_prompt_model("gpt-4o-mini")
        .build()?;
    let store = PgVector::try_new(pool, VECTOR_SIZE as _).await?;

    indexing::Pipeline::from_loader(FileLoader::new(".").with_extensions(&["rs"]))
        .then(MetadataQACode::new(client.clone()))
        .then_chunk(ChunkCode::try_for_language_and_chunk_size(
            "rust",
            10..2048,
        )?)
        .then_in_batch(Embed::new(client).with_batch_size(10))
        .then_store_with(store)
        .run()
        .await?;
    Ok(())
}
