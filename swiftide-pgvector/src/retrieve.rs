use crate::PgVector;
use anyhow::Result;
use async_trait::async_trait;
use pgvector::Vector;
use sqlx::{prelude::FromRow, types::Uuid};
use swiftide_core::{
    querying::{search_strategies::SimilaritySingleEmbedding, states, Query},
    Retrieve,
};
use tracing::info;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
struct RetrievalResult {
    id: Uuid,
    chunk: String,
}

const DEFAULT_LIMIT: usize = 5;

#[async_trait]
impl Retrieve<SimilaritySingleEmbedding<String>> for PgVector {
    #[tracing::instrument]
    async fn retrieve(
        &self,
        search_strategy: &SimilaritySingleEmbedding<String>,
        query: Query<states::Pending>,
    ) -> Result<Query<states::Retrieved>> {
        let Some(embedding) = &query.embedding else {
            anyhow::bail!("No embedding for query")
        };

        let embedding = Vector::from(embedding.clone());
        let pool = self.get_pool();

        let sql = format!(
            "SELECT id, chunk FROM {} ORDER BY embedding <=> $1 LIMIT $2",
            self.table_name
        );
        info!("Running retrieve with SQL: {}", sql);
        let data: Vec<RetrievalResult> = sqlx::query_as(&sql)
            .bind(embedding)
            .bind(DEFAULT_LIMIT as i32)
            .fetch_all(pool)
            .await?;

        let docs = data.into_iter().map(|r| r.chunk).collect();

        Ok(query.retrieved_documents(docs))
    }
}

#[async_trait]
impl Retrieve<SimilaritySingleEmbedding> for PgVector {
    async fn retrieve(
        &self,
        search_strategy: &SimilaritySingleEmbedding,
        query: Query<states::Pending>,
    ) -> Result<Query<states::Retrieved>> {
        Retrieve::<SimilaritySingleEmbedding<String>>::retrieve(
            self,
            &search_strategy.into_concrete_filter::<String>(),
            query,
        )
        .await
    }
}
