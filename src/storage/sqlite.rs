use crate::doc::document::RawDocument;
use crate::doc::project::Project;
use crate::storage::engine::{Engine, EngineError};

use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use std::fs::File;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub struct Sqlite {
    pub url: String,
    pub pool: SqlitePool,
}

impl Sqlite {
    pub async fn setup(url: String) -> Result<Self, sqlx::Error> {
        if url != ":memory:" && !Path::new(&url).exists() {
            println!("sqlite: file does not exist, creating it");
            let _ = File::create(&url)?;
        }
        let pool = SqlitePool::connect(&url).await?;
        Ok(Self { url, pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("migrations/sqlite").run(&self.pool).await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
pub struct DbDocument {
    pub id: Uuid,
    pub project_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub doctype: String,
    pub version: i32,
    pub body: String,
}

impl From<DbDocument> for RawDocument {
    fn from(doc: DbDocument) -> RawDocument {
        RawDocument {
            id: doc.id,
            project_id: doc.project_id,
            owner_id: doc.owner_id,
            name: doc.name,
            doctype: doc.doctype,
            version: doc.version,
            body: doc.body,
        }
    }
}

impl From<&DbDocument> for RawDocument {
    fn from(doc: &DbDocument) -> RawDocument {
        RawDocument {
            id: doc.id,
            project_id: doc.project_id,
            owner_id: doc.owner_id,
            name: doc.name.clone(),
            doctype: doc.doctype.clone(),
            version: doc.version,
            body: doc.body.clone(),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct DbProject {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub version: i32,
    pub body: String,
}

impl From<DbProject> for Project {
    fn from(doc: DbProject) -> Project {
        Project {
            id: doc.id,
            owner_id: doc.owner_id,
            name: doc.name,
            version: doc.version,
            body: doc.body,
        }
    }
}

impl From<sqlx::Error> for EngineError {
    fn from(err: sqlx::Error) -> EngineError {
        match &err {
            sqlx::Error::RowNotFound => EngineError::NotFound,
            _ => EngineError::Storage(err.to_string()),
        }
    }
}

#[async_trait]
impl Engine for Sqlite {
    async fn get_document(&self, id: &Uuid) -> Result<RawDocument, EngineError> {
        let doc = sqlx::query_as::<_, DbDocument>("SELECT * FROM documents WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(doc.into())
    }

    async fn store_document(&self, doc: RawDocument) -> Result<(), EngineError> {
        let _result = sqlx::query(
            "
        INSERT INTO documents (id, project_id, owner_id, name, doctype, version, body)
        VALUES (?, ?, ?, ?, ?, ?, ?);
        ",
        )
        .bind(doc.id)
        .bind(doc.project_id)
        .bind(doc.owner_id)
        .bind(doc.name)
        .bind(doc.doctype)
        .bind(doc.version)
        .bind(doc.body)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_document(&self, doc: RawDocument) -> Result<(), EngineError> {
        let _result = sqlx::query(
            "
        UPDATE documents SET name=? version=? body=? WHERE id=?
        ",
        )
        .bind(doc.name)
        .bind(doc.version)
        .bind(doc.body)
        .bind(doc.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_document(&self, id: &Uuid) -> Result<(), EngineError> {
        sqlx::query("DELETE FROM documents WHERE id=?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_project(&self, id: &Uuid) -> Result<Project, EngineError> {
        let doc = sqlx::query_as::<_, DbProject>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(doc.into())
    }

    async fn store_project(&self, doc: Project) -> Result<(), EngineError> {
        let _result = sqlx::query(
            "
        INSERT INTO projects (id, owner_id, name, version, body)
        VALUES (?, ?, ?, ?, ?);
        ",
        )
        .bind(doc.id)
        .bind(doc.owner_id)
        .bind(doc.name)
        .bind(doc.version)
        .bind(doc.body)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_project(&self, doc: Project) -> Result<(), EngineError> {
        let _result = sqlx::query(
            "
        UPDATE projects SET name=? version=? body=? WHERE id=?
        ",
        )
        .bind(doc.name)
        .bind(doc.version)
        .bind(doc.body)
        .bind(doc.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_project(&self, id: &Uuid) -> Result<(), EngineError> {
        sqlx::query("DELETE FROM projects WHERE id=?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_project_documents(
        &self,
        project_id: &Uuid,
    ) -> Result<Vec<RawDocument>, EngineError> {
        let dbdocs =
            sqlx::query_as::<_, DbDocument>("SELECT * FROM documents WHERE project_id = ?")
                .bind(project_id)
                .fetch_all(&self.pool)
                .await?;

        let docs: Vec<RawDocument> = dbdocs.iter().map(|e| e.into()).collect();

        Ok(docs)
    }
}

#[cfg(test)]
mod test {
    #[async_std::test]
    async fn test_sqlite() -> std::io::Result<()> {
        let storage = super::Sqlite::setup(":memory:".into())
            .await
            .expect("Database to be initialized in memory");
        let _ = storage.migrate().await.expect("Database to be migrated");
        Ok(())
    }
}
