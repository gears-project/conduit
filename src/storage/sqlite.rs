use crate::doc::change::Change;
use crate::doc::document::{DocType, RawDocument};
use crate::doc::project::{Project, ProjectField};
use crate::storage::engine::{Engine, EngineError, QueryRequest, QueryResponse, QueryResponseMeta};

use async_trait::async_trait;
// use sqlx::sqlite::{SqlitePool, SqliteConnectOptions, SqliteJournalMode};
use sqlx::sqlite::{SqlitePool};
use std::fs::File;
use std::path::Path;
use uuid::Uuid;

use crate::doc::common::DateTime;

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
        /*
        use sqlx::ConnectOptions;
        let mut options = SqliteConnectOptions::new();
        options.log_statements(tracing::log::LevelFilter::Trace);
        options.journal_mode(SqliteJournalMode::Wal);
        let pool = SqlitePool::connect_with(options).await?;
        */

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
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<RawDocument> for DbDocument {
    fn from(doc: RawDocument) -> DbDocument {
        DbDocument {
            id: doc.id,
            project_id: doc.project_id,
            owner_id: doc.owner_id,
            name: doc.name,
            doctype: doc.doctype,
            version: doc.version,
            body: serde_json::to_string(&doc.body).expect("Body to be serializable"),
            created_at: doc.created_at,
            updated_at: doc.updated_at,
        }
    }
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
            body: serde_json::from_str(&doc.body).expect("Body to be deserializable"),
            created_at: doc.created_at,
            updated_at: doc.updated_at,
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
            body: serde_json::from_str(&doc.body).expect("Body to be deserializable"),
            created_at: doc.created_at,
            updated_at: doc.updated_at,
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
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<DbProject> for Project {
    fn from(doc: DbProject) -> Project {
        Project {
            id: doc.id,
            owner_id: doc.owner_id,
            name: doc.name,
            version: doc.version,
            body: doc.body,
            created_at: doc.created_at,
            updated_at: doc.updated_at,
        }
    }
}

impl From<&DbProject> for Project {
    fn from(doc: &DbProject) -> Project {
        Project {
            id: doc.id,
            owner_id: doc.owner_id,
            name: doc.name.clone(),
            version: doc.version,
            body: doc.body.clone(),
            created_at: doc.created_at,
            updated_at: doc.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct DbChange {
    pub id: i32,
    pub version: i32,
    pub forward: String,
    pub reverse: String,
    pub document_id: Uuid,
}

impl From<Change> for DbChange {
    fn from(change: Change) -> DbChange {
        DbChange {
            id: change.id,
            version: change.version,
            forward: serde_json::to_string(&change.forward).expect("Patch to be serializable"),
            reverse: serde_json::to_string(&change.reverse).expect("Patch to be serializable"),
            document_id: change.document_id,
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
        let doc: DbDocument = doc.into();
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

    async fn update_document(
        &self,
        doc: RawDocument,
        change: Option<Change>,
    ) -> Result<(), EngineError> {
        let mut tx = self.pool.begin().await?;

        let doc: DbDocument = doc.into();

        let _result = sqlx::query(
            "
        UPDATE documents SET name=?, version=?, body=? WHERE id=?
        ",
        )
        .bind(doc.name)
        .bind(doc.version)
        .bind(doc.body)
        .bind(doc.id)
        .execute(&mut tx)
        .await?;

        if let Some(change) = change {
            let dbchange: DbChange = change.into();
            let _result = sqlx::query(
                "
            INSERT INTO changes (document_id, version, forward, reverse)
            VALUES (?, ?, ?, ?);
            ",
            )
            .bind(dbchange.document_id)
            .bind(dbchange.version)
            .bind(dbchange.forward)
            .bind(dbchange.reverse)
            .execute(&mut tx)
            .await?;
        }

        let _ = tx.commit().await?;

        Ok(())
    }

    async fn delete_document(&self, id: &Uuid) -> Result<(), EngineError> {
        sqlx::query("DELETE FROM documents WHERE id=?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_projects(
        &self,
        params: Option<QueryRequest<ProjectField>>,
    ) -> Result<QueryResponse<Project>, EngineError> {

        use std::fmt::Write;
        let mut query = "SELECT * FROM projects".to_string();
        let query_agg = "SELECT COUNT(*) FROM projects".to_string();

        let mut limit = 100;
        let mut offset = 0;

        match params {
            Some(params) => {
                match params.page {
                    Some(page) => {
                        match page.limit {
                            Some(l) => {
                                limit = l
                            }
                            None => { }
                        }
                        match page.offset {
                            Some(o) => {
                                offset = o
                            }
                            None => { }
                        }
                    }
                    None => { }
                }
            }
            None => { }
        };

        if limit != 0 {
            write!(query, " LIMIT {} ", limit).expect("String operation to succeed");
        }
        write!(query, " OFFSET {} ", offset).expect("String operation to succeed");

        let dbdocs = sqlx::query_as::<_, DbProject>(&query)
            .fetch_all(&self.pool)
            .await?;

        let docs: Vec<Project> = dbdocs.iter().map(|e| e.into()).collect();

        let agg = sqlx::query!(
            "SELECT COUNT(*) as count FROM projects"
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(QueryResponse::<Project> {
            data: docs,
            meta: QueryResponseMeta {
                offset: Some(offset),
                total: Some(agg.count),
            },
        })
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
        let _result = sqlx::query!(
            "
        UPDATE projects SET name=?, version=?, body=? WHERE id=?
        ",
            doc.name,
            doc.version,
            doc.body,
            doc.id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /*
    async fn update_project(&self, doc: Project) -> Result<(), EngineError> {
        let _result = sqlx::query(
            "
        UPDATE projects SET name=?, version=?, body=? WHERE id=?
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
    */

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
        variant: DocType,
    ) -> Result<Vec<RawDocument>, EngineError> {
        let dbdocs = sqlx::query_as::<_, DbDocument>(
            "SELECT * FROM documents WHERE doctype = ? AND project_id = ?",
        )
        .bind(variant.to_string())
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        let docs: Vec<RawDocument> = dbdocs.iter().map(|e| e.into()).collect();

        Ok(docs)
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_sqlite() -> std::io::Result<()> {
        let storage = super::Sqlite::setup(":memory:".into())
            .await
            .expect("Database to be initialized in memory");
        let _ = storage.migrate().await.expect("Database to be migrated");
        Ok(())
    }
}
