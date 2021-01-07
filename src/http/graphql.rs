use crate::doc::document::RawDocument;
use crate::doc::project::Project;
use crate::storage::engine::EngineContainer;
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

pub struct Query;

#[async_graphql::Object]
impl Project {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn version(&self) -> &i32 {
        &self.version
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn body(&self) -> &str {
        &self.body
    }

    async fn documents(&self, ctx: &Context<'_>) -> FieldResult<Vec<RawDocument>> {
        let storage = ctx.data::<EngineContainer>().expect("To get a container");
        let docs = storage.get_project_documents(&self.id).await?;
        Ok(docs)
    }
}

#[async_graphql::Object]
impl RawDocument {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn version(&self) -> &i32 {
        &self.version
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn body(&self) -> &str {
        &self.body
    }
}

#[Object]
impl Query {
    #[cfg(test)]
    pub async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> FieldResult<Project> {
        let storage = ctx.data::<EngineContainer>().expect("To get a container");
        let project = storage.get_project(&id).await?;
        Ok(project)
    }
}

#[cfg(test)]
mod test {

    use assert_json_diff::assert_json_eq;
    use async_graphql::*;
    use serde_json::json;

    #[async_std::test]
    async fn test_schema() -> std::io::Result<()> {
        let schema = Schema::new(super::Query, EmptyMutation, EmptySubscription);
        let res = schema.execute("{ add(a: 10, b: 20) }").await;
        assert_json_eq!(
            res,
            json!({
                "data": {
                    "add": 30
                }
            })
        );

        Ok(())
    }
}
