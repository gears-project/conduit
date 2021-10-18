use async_graphql::extensions::{ApolloTracing, Logger};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Schema, EmptySubscription};
use async_graphql::{Request, Response};

use axum::response::IntoResponse;
use axum::{extract::Extension, handler::get, response::Html, AddExtensionLayer, Json, Router};

use std::env;

use super::graphql::{MutationRoot, Query};
use crate::storage::engine::{Engine, EngineContainer};
use crate::storage::sqlite::Sqlite;

async fn graphql_handler(schema: Extension<Schema<Query, MutationRoot, EmptySubscription>>, req: Json<Request>) -> Json<Response> {
        schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub async fn serve() -> Result<(), ()> {
    run().await
}

async fn run() -> Result<(), ()> {
    env_logger::init();
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

    // let storage = Sqlite::setup(":memory:".into())
    let storage = Sqlite::setup("test.db".into())
        .await
        .expect("The sqlite storage to be set up");
    let _ = storage
        .migrate()
        .await
        .expect("The sqlite storage to be migrated");

    let project = crate::doc::project::Project::new(crate::util::naming::empty_uuid());

    let _ = storage
        .store_project(project.clone())
        .await
        .expect("The project to be inserted");

    let engine = EngineContainer::new(storage);

    let schema = Schema::build(Query, MutationRoot, EmptySubscription)
        .extension(Logger)
        .extension(ApolloTracing)
        .data(engine)
        .finish();

    println!("Playground: http://{}", listen_addr);

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();

    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use async_std::prelude::*;
    use serde_json::json;
    use std::time::Duration;

    #[test]
    fn sample() -> Result<()> {
        task::block_on(async {
            let listen_addr = find_listen_addr().await;
            env::set_var("LISTEN_ADDR", format!("{}", listen_addr));

            let server: task::JoinHandle<Result<()>> = task::spawn(async move {
                run().await?;

                Ok(())
            });

            let client: task::JoinHandle<Result<()>> = task::spawn(async move {
                let listen_addr = env::var("LISTEN_ADDR").unwrap();

                task::sleep(Duration::from_millis(300)).await;

                let string = surf::post(format!("http://{}/graphql", listen_addr))
                    .body(Body::from(r#"{"query":"{ human(id:\"1000\") {name} }"}"#))
                    .header("Content-Type", "application/json")
                    .recv_string()
                    .await?;

                assert_eq!(
                    string,
                    json!({"data":{"human":{"name":"Luke Skywalker"}}}).to_string()
                );

                Ok(())
            });

            server.race(client).await?;

            Ok(())
        })
    }

    async fn find_listen_addr() -> async_std::net::SocketAddr {
        async_std::net::TcpListener::bind("localhost:0")
            .await
            .unwrap()
            .local_addr()
            .unwrap()
    }
}
*/
