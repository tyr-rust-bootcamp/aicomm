mod persist;
mod retrieve;

use anyhow::Result;
use derive_builder::Builder;
use sqlx::PgPool;
use swiftide_core::Persist;

#[derive(Builder, Debug, Clone)]
pub struct PgVector {
    /// The database connection pool.
    pool: PgPool,
    /// The name of the table to store the vectors in.
    #[builder(default = "String::from(\"swiftide_rag\")")]
    table_name: String,
    /// The size of the vectors to store in the database.
    vector_size: i32,
    /// The batch size to use when storing nodes.
    #[builder(default = "128")]
    batch_size: usize,
}

impl PgVector {
    pub async fn try_new(pool: PgPool, vector_size: i32) -> Result<Self> {
        let vector = PgVectorBuilder::default()
            .pool(pool)
            .vector_size(vector_size)
            .build()?;

        vector.setup().await?;
        Ok(vector)
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
