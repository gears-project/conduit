use crate::doc::document::RawDocument;
use crate::doc::project::Project;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Engine {
    async fn get_document(&self, id: Uuid) -> Result<RawDocument, EngineError>;
    async fn store_document(&self, doc: RawDocument) -> Result<(), EngineError>;
    async fn update_document(&self, doc: RawDocument) -> Result<(), EngineError>;

    async fn get_project(&self, id: Uuid) -> Result<Project, EngineError>;
    async fn store_project(&self, doc: Project) -> Result<(), EngineError>;
    async fn update_project(&self, doc: Project) -> Result<(), EngineError>;
}

#[derive(Debug, PartialEq)]
pub enum EngineError {
    Storage(String),
}
