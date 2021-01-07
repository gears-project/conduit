extern crate conduit;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use conduit::doc::project::Project;
use conduit::http::graphql::Query;
use conduit::storage::engine::{Engine, EngineContainer};
use conduit::storage::sqlite::Sqlite;

use assert_json_diff::assert_json_eq;
use serde_json::json;

#[async_std::test]
async fn test_graphql_schema() -> std::io::Result<()> {
    let _ = env_logger::try_init();

    let storage = Sqlite::setup(":memory:".into())
        .await
        .expect("The sqlite storage to be set up");
    let _ = storage
        .migrate()
        .await
        .expect("The sqlite storage to be migrated");

    let project = Project::new(conduit::util::naming::empty_uuid());
    let project_id = project.id.to_hyphenated().to_string();

    let _ = storage
        .store_project(project.clone())
        .await
        .expect("The project to be inserted");

    let engine = EngineContainer::new(storage);

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(engine)
        .finish();

    let res = schema
        .execute(format!(
            "{{
                project (id:\"{}\") {{
                    id,
                    name,
                    version,
                    body
                }}
            }}",
            project_id
        ))
        .await;
    assert_json_eq!(
        res,
        json!({
            "data": {
                "project": {
                    "id": project_id,
                    "name": "New Project",
                    "version": 0,
                    "body": ""
                }
            }
        })
    );

    Ok(())
}
