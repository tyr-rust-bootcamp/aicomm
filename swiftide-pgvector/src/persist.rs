use crate::PgVector;
use anyhow::Result;
use async_trait::async_trait;
use pgvector::Vector;
use swiftide_core::{
    indexing::{EmbeddedField, IndexingStream, Node},
    Persist,
};
use tracing::info;

#[async_trait]
impl Persist for PgVector {
    #[tracing::instrument(skip_all)]
    async fn setup(&self) -> Result<()> {
        let pool = self.get_pool();
        let mut tx = pool.begin().await?;

        // create extension
        let sql = "CREATE EXTENSION IF NOT EXISTS vector";
        sqlx::query(sql).execute(&mut *tx).await?;

        // create table
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id UUID PRIMARY KEY,
            path VARCHAR NOT NULL,
            chunk TEXT NOT NULL,
            metadata JSONB NOT NULL,
            embedding VECTOR({}),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)
        ",
            self.table_name, self.vector_size
        );
        sqlx::query(&sql).execute(&mut *tx).await?;

        // create hnsw index
        let sql = format!(
            "CREATE INDEX IF NOT EXISTS {}_embedding_idx ON {} USING hnsw (embedding vector_cosine_ops)",
            self.table_name, self.table_name
        );
        sqlx::query(&sql).execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn store(&self, node: Node) -> Result<Node> {
        let mut nodes = vec![node; 1];
        self.store_nodes(&nodes).await?;

        let node = nodes.swap_remove(0);

        Ok(node)
    }

    #[tracing::instrument(skip_all)]
    async fn batch_store(&self, nodes: Vec<Node>) -> IndexingStream {
        self.store_nodes(&nodes).await.map(|()| nodes).into()
    }

    fn batch_size(&self) -> Option<usize> {
        Some(self.batch_size)
    }
}

impl PgVector {
    async fn store_nodes(&self, nodes: &[Node]) -> Result<()> {
        let pool = self.get_pool();
        let mut tx = pool.begin().await?;

        for node in nodes {
            info!("storing node: {:?}", node);
            let id = node.id();
            let path = node.path.to_string_lossy();
            let chunk = &node.chunk;
            let metadata = serde_json::to_value(&node.metadata)?;
            let data = node
                .vectors
                .as_ref()
                // TODO: verify compiler optimizes the double loops away
                .and_then(|v| v.get(&EmbeddedField::Combined))
                .map(|v| v.to_vec())
                .unwrap_or_default();

            let sql = format!(
                "INSERT INTO {} (id, path, chunk, metadata, embedding) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET (path, chunk, metadata, embedding, updated_at) = ($2, $3, $4, $5, CURRENT_TIMESTAMP)",
                self.table_name
            );
            sqlx::query(&sql)
                .bind(id)
                .bind(path)
                .bind(chunk)
                .bind(metadata)
                .bind(Vector::from(data))
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
