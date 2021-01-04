use crate::doc::document::RawDocument;
use crate::storage::engine::{Engine, EngineError};

use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use std::fs::File;
use std::path::Path;
use uuid::Uuid;

pub struct Sqlite {
    pub url: String,
    pub pool: SqlitePool,
}

impl Sqlite {
    pub async fn setup(url: String) -> Result<Self, sqlx::Error> {

        if url != ":memory:" {
            if !Path::new(&url).exists() {
                println!("sqlite: file does not exist, creating it");
                let _ = File::create(&url)?;
            }
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
    pub name: String,
    pub doctype: String,
    pub version: i32,
    pub body: String,
}

impl From<DbDocument> for RawDocument {
    fn from(doc: DbDocument) -> RawDocument {
        RawDocument {
            id: doc.id,
            name: doc.name,
            doctype: doc.doctype,
            version: doc.version,
            body: doc.body,
        }
    }
}

impl From<sqlx::Error> for EngineError {
    fn from(err: sqlx::Error) -> EngineError {
        EngineError::Storage(err.to_string())
    }
}

#[async_trait]
impl Engine for Sqlite {
    async fn get_document(&self, id: Uuid) -> Result<RawDocument, EngineError> {
        let doc = sqlx::query_as::<_, DbDocument>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(doc.into())
    }
    async fn store_document(&self, doc: RawDocument) -> Result<(), EngineError> {
        let _result = sqlx::query(
            "
        INSERT INTO documents (id, name, doctype, version, body)
        VALUES (?, ?, ?, ?, ?);
        ",
        )
        .bind(doc.id)
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
}

#[cfg(test)]
mod test {
    #[async_std::test]
    async fn test_sqlite() -> std::io::Result<()> {
        let storage = super::Sqlite::setup(":memory:".into())
            .await
            .unwrap();
        let _ = storage.migrate().await.unwrap();
        Ok(())
    }
}
