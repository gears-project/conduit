use sqlx::sqlite::SqlitePool;
use uuid::Uuid;
use async_trait::async_trait;
use crate::doc::document::RawDocument;
use crate::storage::engine::Engine;

pub struct Sqlite {
    pub url: String,
    pub pool: SqlitePool,
}

impl Sqlite {
    pub async fn setup(url: String) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        Ok(
            Self {
                url,
                pool,
            }
        )
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

#[async_trait]
impl Engine for Sqlite {
    async fn get_document(&self, id: Uuid) -> Result<RawDocument, sqlx::Error> {

        let doc = sqlx::query_as::<_, DbDocument>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(doc.into())
    }
    async fn store_document(&self, doc:RawDocument) -> Result<(), sqlx::Error> {
        let _result = sqlx::query("
        INSERT INTO documents (id, name, doctype, version, body)
        VALUES (?, ?, ?, ?, ?);
        ")
            .bind(doc.id)
            .bind(doc.name)
            .bind(doc.doctype)
            .bind(doc.version)
            .bind(doc.body)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn update_document(&self, doc:RawDocument) -> Result<(), sqlx::Error> {
        let _result = sqlx::query("
        UPDATE documents SET name=? version=? body=? WHERE id=?
        ")
            .bind(doc.name)
            .bind(doc.version)
            .bind(doc.body)
            .bind(doc.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}


