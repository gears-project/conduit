use std::fmt;

use crate::doc::document::{DocType, RawDocument};
use crate::doc::project::Project;
use crate::doc::change::Change;
use async_trait::async_trait;
use uuid::Uuid;
use json_patch::diff;

#[async_trait]
pub trait Engine: Send + Sync {
    async fn get_document(&self, id: &Uuid) -> Result<RawDocument, EngineError>;
    async fn store_document(&self, doc: RawDocument) -> Result<(), EngineError>;
    async fn update_document(&self, doc: RawDocument) -> Result<(), EngineError>;
    async fn delete_document(&self, id: &Uuid) -> Result<(), EngineError>;

    async fn get_projects(&self) -> Result<Vec<Project>, EngineError>;
    async fn get_project(&self, id: &Uuid) -> Result<Project, EngineError>;
    async fn store_project(&self, doc: Project) -> Result<(), EngineError>;
    async fn update_project(&self, doc: Project) -> Result<(), EngineError>;
    async fn delete_project(&self, id: &Uuid) -> Result<(), EngineError>;

    async fn add_change(&self, change: &Change) -> Result<(), EngineError>;

    async fn get_project_documents(
        &self,
        project_id: &Uuid,
        variant: DocType,
    ) -> Result<Vec<RawDocument>, EngineError>;
}

#[derive(Debug, PartialEq)]
pub enum EngineError {
    NotFound,
    Storage(String),
    VersionMismatch(i32, i32),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineError::NotFound => write!(f, "Not Found"),
            EngineError::Storage(s) => write!(f, "Error : {}", s),
            EngineError::VersionMismatch(a, b) => write!(f, "Document version mismatch : {}, {}", a, b),
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
    pub async fn update_document(&self, doc: &mut RawDocument) -> Result<(), EngineError> {
        let current_doc = self.get_document(&doc.id).await?;
        if current_doc.version != doc.version {
            Err(EngineError::VersionMismatch(doc.version, current_doc.version))
        } else {
            doc.change();
            let forward = diff(&current_doc.body, &doc.body);
            let reverse = diff(&doc.body, &current_doc.body);
            let _ = self.engine.add_change(&Change {
                id: 0,
                document_id: doc.id,
                version: doc.version,
                forward: serde_json::to_value(forward).expect("Patch to convert to Value"),
                reverse: serde_json::to_value(reverse).expect("Patch to convert to Value"),
            });
            self.engine.update_document(doc.clone()).await
        }
    }
    pub async fn delete_document(&self, id: &Uuid) -> Result<(), EngineError> {
        self.engine.delete_document(id).await
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, EngineError> {
        self.engine.get_projects().await
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

    pub async fn get_project_documents(
        &self,
        project_id: &Uuid,
        variant: DocType,
    ) -> Result<Vec<RawDocument>, EngineError> {
        self.engine.get_project_documents(project_id, variant).await
    }
}

#[derive(Debug)]
pub enum EngineVariant {
    Sqlite(super::sqlite::Sqlite),
}
