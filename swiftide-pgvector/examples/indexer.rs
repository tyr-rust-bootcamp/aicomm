use anyhow::Result;
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

const VECTOR_SIZE: usize = 384;

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/swiftide_rag")
        .await?;
    let fastembed = integrations::fastembed::FastEmbed::try_default()?;
    let client = integrations::ollama::Ollama::default()
        .with_default_prompt_model("llama3.2")
        .to_owned();
    let store = PgVector::try_new(pool, VECTOR_SIZE as _).await?;

    indexing::Pipeline::from_loader(FileLoader::new(".").with_extensions(&["rs"]))
        .then(MetadataQACode::new(client.clone()))
        .then_chunk(ChunkCode::try_for_language_and_chunk_size(
            "rust",
            10..2048,
        )?)
        .then_in_batch(Embed::new(fastembed).with_batch_size(10))
        .then_store_with(store)
        .run()
        .await?;
    Ok(())
}
