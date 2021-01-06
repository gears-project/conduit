use std::fmt;

use crate::doc::document::RawDocument;
use crate::doc::project::Project;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Engine: Send + Sync {
    async fn get_document(&self, id: &Uuid) -> Result<RawDocument, EngineError>;
    async fn store_document(&self, doc: RawDocument) -> Result<(), EngineError>;
    async fn update_document(&self, doc: RawDocument) -> Result<(), EngineError>;
    async fn delete_document(&self, id: &Uuid) -> Result<(), EngineError>;

    async fn get_project(&self, id: &Uuid) -> Result<Project, EngineError>;
    async fn store_project(&self, doc: Project) -> Result<(), EngineError>;
    async fn update_project(&self, doc: Project) -> Result<(), EngineError>;
    async fn delete_project(&self, id: &Uuid) -> Result<(), EngineError>;
}

#[derive(Debug, PartialEq)]
pub enum EngineError {
    NotFound,
    Storage(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineError::NotFound => write!(f, "Not Found"),
            EngineError::Storage(s) => write!(f, "Error : {}", s),
        }
    }
}

pub struct EngineContainer {
    engine: Box<dyn Engine>,
}

impl EngineContainer {
    pub fn new(engine: impl Engine + 'static) -> Self {
        Self {
            engine: Box::new(engine),
        }
    }

    pub async fn get_document(&self, id: &Uuid) -> Result<RawDocument, EngineError> {
        self.engine.get_document(id).await
    }
    pub async fn store_document(&self, doc: RawDocument) -> Result<(), EngineError> {
        self.engine.store_document(doc).await
    }
    pub async fn update_document(&self, doc: RawDocument) -> Result<(), EngineError> {
        self.engine.update_document(doc).await
    }
    pub async fn delete_document(&self, id: &Uuid) -> Result<(), EngineError> {
        self.engine.delete_document(id).await
    }

    pub async fn get_project(&self, id: &Uuid) -> Result<Project, EngineError> {
        self.engine.get_project(id).await
    }
    pub async fn store_project(&self, doc: Project) -> Result<(), EngineError> {
        self.engine.store_project(doc).await
    }
    pub async fn update_project(&self, doc: Project) -> Result<(), EngineError> {
        self.engine.update_project(doc).await
    }
    pub async fn delete_project(&self, id: &Uuid) -> Result<(), EngineError> {
        self.engine.delete_project(id).await
    }
}

#[derive(Debug)]
pub enum EngineVariant {
    Sqlite(super::sqlite::Sqlite),
}
