extern crate conduit;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use conduit::doc::document::DigraphDocument;
use conduit::doc::project::Project;
use conduit::http::graphql::{MutationRoot, Query};
use conduit::storage::engine::{Engine, EngineContainer};
use conduit::storage::sqlite::Sqlite;

use assert_json_diff::{assert_json_eq, assert_json_include};
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

#[async_std::test]
async fn test_graphql_schema_query_project_documents() -> std::io::Result<()> {
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

    let doc1 = DigraphDocument::create(&project);
    let doc1_id = doc1.id.to_hyphenated().to_string();

    println!("DOC PROJECT OWNER IS {}", doc1.project_id);

    let doc2 = DigraphDocument::create(&project);
    let doc2_id = doc2.id.to_hyphenated().to_string();

    let _ = storage
        .store_document(doc1.into())
        .await
        .expect("The document to be inserted");

    let _ = storage
        .store_document(doc2.into())
        .await
        .expect("The document to be inserted");

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
                    body,
                    digraphs {{
                        id,
                        name,
                        version
                        body {{
                            nodes {{
                                id
                            }}
                            links {{
                                id
                            }}
                        }}
                    }}
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
                    "body": "",
                    "digraphs" : [
                        {
                            "id": doc1_id,
                            "name": "New",
                            "version": 0,
                            "body": {
                                "nodes": [],
                                "links": []
                            }
                        },
                        {
                            "id": doc2_id,
                            "name": "New",
                            "version": 0,
                            "body": {
                                "nodes": [],
                                "links": []
                            }
                        }
                    ]
                }
            }
        })
    );

    Ok(())
}

#[async_std::test]
async fn test_graphql_schema_digraph_operations() -> std::io::Result<()> {
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

    let schema = Schema::build(Query, MutationRoot, EmptySubscription)
        .data(engine)
        .finish();

    let create_digraph_res = schema
        .execute(format!(
            "
            mutation create {{
              digraphCreate(projectId: \"{}\") {{
                id
                name
              }}
            }}",
            project_id
        ))
        .await;

    let create_digraph_res_json = serde_json::to_value(create_digraph_res)
        .expect("GraphQL response to be deserializable to Value");
    assert_json_include!(
        actual: create_digraph_res_json.clone(),
        expected: json!({
            "data": {
                "digraphCreate": {
                    "name": "New"
                }
            }
        })
    );

    let doc_id = serde_json::to_string(
        create_digraph_res_json
            .pointer("/data/digraphCreate/id")
            .expect("Doc ID to exist in graphql response"),
    )
    .expect("Doc ID value to be deserializable");

    let create_node_res = schema
        .execute(format!(
            "
            mutation add {{
              digraphAddNode(
                projectId: \"{}\",
                docId: {}
              ) {{
                id
                name
                body {{
                  nodes {{
                    id
                    name
                  }}
                }}
              }}
            }}",
            project_id, doc_id
        ))
        .await;

    let create_node_res_json = serde_json::to_value(create_node_res)
        .expect("GraphQL response to be deserializable to Value");
    assert_json_include!(
        actual: create_node_res_json.clone(),
        expected: json!({
            "data": {
                "digraphAddNode": {
                    "name": "New",
                    "body": {
                        "nodes": [
                            {
                                "id": 1
                            }
                        ]
                    }

                }
            }
        })
    );

    let node_id = serde_json::to_string(
        create_node_res_json
            .pointer("/data/digraphAddNode/body/nodes/0/id")
            .expect("Node ID to exist in graphql response"),
    )
    .expect("Node ID value to be deserializable");

    let update_node_res = schema
        .execute(format!(
            "
            mutation update {{
              digraphUpdateNode(
                projectId: \"{}\",
                docId: {},
                nodeId: {},
              ) {{
                id
                name
                body {{
                  nodes {{
                    id
                    name
                  }}
                }}
              }}
            }}",
            project_id, doc_id, node_id
        ))
        .await;

    let update_node_res_json = serde_json::to_value(update_node_res)
        .expect("GraphQL response to be deserializable to Value");

    assert_json_include!(
        actual: update_node_res_json.clone(),
        expected: json!({
            "data": {
                "digraphUpdateNode": {
                    "name": "New"
                }
            }
        })
    );

    let updated_node_id = serde_json::to_string(
        update_node_res_json
            .pointer("/data/digraphUpdateNode/body/nodes/0/id")
            .expect("Node ID to exist in graphql response"),
    )
    .expect("Node ID value to be deserializable");

    assert_eq!(updated_node_id, "1".to_string());

    Ok(())
}
