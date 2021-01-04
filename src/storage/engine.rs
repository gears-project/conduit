use crate::doc::document::RawDocument;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Engine {
    async fn get_document(&self, id: Uuid) -> Result<RawDocument, EngineError>;
    async fn store_document(&self, doc: RawDocument) -> Result<(), EngineError>;
    async fn update_document(&self, doc: RawDocument) -> Result<(), EngineError>;
}

pub enum EngineError {
    Storage(String),
}
