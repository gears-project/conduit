use crate::doc::document::{DocType, RawDocument};
use crate::doc::project::Project;
use crate::storage::engine::{EngineContainer, EngineError};
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

macro_rules! register_graphql_doc {
    ($doc:ty, $body:ty) => {
        #[async_graphql::Object]
        impl $doc {
            async fn id(&self) -> &Uuid {
                &self.id
            }

            async fn version(&self) -> &i32 {
                &self.version
            }

            async fn name(&self) -> &str {
                &self.name
            }

            async fn body(&self) -> &$body {
                &self.body
            }
        }
    };
}

use crate::doc::document::DigraphDocument;
use crate::model::digraph::{Digraph, DigraphMessage, LinkSettings, NodeSettings};

register_graphql_doc!(DigraphDocument, Digraph);

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

    async fn digraphs(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<crate::doc::document::DigraphDocument>> {
        let storage = ctx.data::<EngineContainer>().expect("To get a container");
        let docs = storage
            .get_project_documents(&self.id, DocType::Digraph)
            .await?;
        Ok(docs.iter().map(|doc| doc.into()).collect())
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
}

pub struct Query;

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

    async fn projects(&self, ctx: &Context<'_>) -> FieldResult<Vec<Project>> {
        let storage = ctx.data::<EngineContainer>().expect("To get a container");
        let response = storage.get_projects(None).await?;
        Ok(response.data)
    }
}

pub struct MutationRoot;

async fn digraph_change(
    ctx: &Context<'_>,
    project_id: Uuid,
    doc_id: Uuid,
    msg: DigraphMessage,
) -> Result<DigraphDocument, EngineError> {
    let storage = ctx.data::<EngineContainer>().expect("To get a container");
    let _project = storage.get_project(&project_id).await?;
    let mut doc: DigraphDocument = storage.get_document(&doc_id).await?.into();

    if let Err(err) = doc.body.message(msg) {
        Err(EngineError::Storage(format!("{}", err)))
    } else {
        let _ = storage.update_document(&mut doc.clone().into()).await?;
        Ok(doc)
    }
}

#[Object]
impl MutationRoot {
    async fn digraph_create(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
    ) -> FieldResult<DigraphDocument> {
        let storage = ctx.data::<EngineContainer>().expect("To get a container");

        let project = storage.get_project(&project_id).await?;
        let doc = DigraphDocument::create(&project);
        let _ = storage.store_document(doc.clone().into()).await?;
        Ok(doc)
    }

    async fn digraph_add_node(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        doc_id: Uuid,
        attrs: Option<NodeSettings>,
    ) -> FieldResult<DigraphDocument> {
        use crate::model::digraph::DigraphMessage;
        let msg = DigraphMessage::AddNode(attrs.unwrap_or_default());

        Ok(digraph_change(ctx, project_id, doc_id, msg).await?)
    }

    async fn digraph_update_node(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        doc_id: Uuid,
        node_id: i32,
        attrs: Option<NodeSettings>,
    ) -> FieldResult<DigraphDocument> {
        use crate::model::digraph::DigraphMessage;
        let msg = DigraphMessage::UpdateNode(node_id, attrs.unwrap_or_default());

        Ok(digraph_change(ctx, project_id, doc_id, msg).await?)
    }

    async fn digraph_remove_node(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        doc_id: Uuid,
        node_id: i32,
    ) -> FieldResult<DigraphDocument> {
        use crate::model::digraph::DigraphMessage;
        let msg = DigraphMessage::RemoveNode(node_id);

        Ok(digraph_change(ctx, project_id, doc_id, msg).await?)
    }

    async fn digraph_add_link(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        doc_id: Uuid,
        source_id: i32,
        target_id: i32,
        attrs: Option<LinkSettings>,
    ) -> FieldResult<DigraphDocument> {
        use crate::model::digraph::DigraphMessage;
        let msg = DigraphMessage::AddLink(source_id, target_id, attrs.unwrap_or_default());

        Ok(digraph_change(ctx, project_id, doc_id, msg).await?)
    }

    async fn digraph_update_link(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        doc_id: Uuid,
        link_id: i32,
        attrs: Option<LinkSettings>,
    ) -> FieldResult<DigraphDocument> {
        use crate::model::digraph::DigraphMessage;
        let msg = DigraphMessage::UpdateLink(link_id, attrs.unwrap_or_default());

        Ok(digraph_change(ctx, project_id, doc_id, msg).await?)
    }

    async fn digraph_remove_link(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        doc_id: Uuid,
        link_id: i32,
    ) -> FieldResult<DigraphDocument> {
        use crate::model::digraph::DigraphMessage;
        let msg = DigraphMessage::RemoveLink(link_id);

        Ok(digraph_change(ctx, project_id, doc_id, msg).await?)
    }
}

#[cfg(test)]
mod test {

    use assert_json_diff::assert_json_eq;
    use async_graphql::*;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_schema() -> std::io::Result<()> {
        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
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
