use uuid::Uuid;
use async_trait::async_trait;
use crate::doc::document::RawDocument;

#[async_trait]
pub trait Engine {
    async fn get_document(&self, id: Uuid) -> Result<RawDocument, sqlx::Error>;
    async fn store_document(&self, doc:RawDocument) -> Result<(), sqlx::Error>;
}
