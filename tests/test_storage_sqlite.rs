use tempdir::TempDir;

extern crate conduit;
use conduit::storage::sqlite::Sqlite;

#[tokio::test]
async fn test_sqlite_in_memory() -> std::io::Result<()> {
    let _ = env_logger::try_init();
    let storage = Sqlite::setup(":memory:".into())
        .await
        .expect("The sqlite storage to be set up");
    storage
        .migrate()
        .await
        .expect("The sqlite storage to be migrated");
    Ok(())
}

#[tokio::test]
async fn test_sqlite_on_disk() -> std::io::Result<()> {
    let _ = env_logger::try_init();
    let dir = TempDir::new("").expect("To be able to create a temporary directory");
    let file_path = dir.path().join("test.db");

    let storage = Sqlite::setup(
        file_path
            .into_os_string()
            .into_string()
            .expect("A path to be created"),
    )
    .await
    .expect("The sqlite storage to be set up");
    storage.migrate().await.expect("Migrations to be run");

    Ok(())
}

#[tokio::test]
async fn test_sqlite_engine_functions() -> std::io::Result<()> {
    use conduit::doc::document::{DigraphDocument, RawDocument};
    use conduit::doc::project::Project;
    use conduit::storage::engine::Engine;

    let _ = env_logger::try_init();
    let storage = Sqlite::setup(":memory:".into())
        .await
        .expect("The sqlite storage to be set up");

    let _ = storage
        .migrate()
        .await
        .expect("The sqlite storage to be migrated");

    let project = Project::new(conduit::util::naming::empty_uuid());

    let _ = storage
        .store_project(project.clone())
        .await
        .expect("The project to be inserted");

    let doc = DigraphDocument::create(&project);

    let raw_doc: RawDocument = doc.clone().into();

    let _ = storage
        .store_document(raw_doc.clone())
        .await
        .expect("The document to be inserted");

    let retrieved_raw_doc = storage
        .get_document(&doc.id)
        .await
        .expect("The stored document to be retrieved");

    assert_eq!(raw_doc.id, retrieved_raw_doc.id);
    assert_eq!(raw_doc.project_id, retrieved_raw_doc.project_id);
    assert_eq!(raw_doc.owner_id, retrieved_raw_doc.owner_id);
    assert_eq!(raw_doc.name, retrieved_raw_doc.name);
    assert_eq!(raw_doc.doctype, retrieved_raw_doc.doctype);
    assert_eq!(raw_doc.version, retrieved_raw_doc.version);
    assert_eq!(raw_doc.body, retrieved_raw_doc.body);

    let doc_retrieved: DigraphDocument = retrieved_raw_doc.into();

    assert_eq!(doc.id, doc_retrieved.id);
    assert_eq!(doc.project_id, doc_retrieved.project_id);
    assert_eq!(doc.owner_id, doc_retrieved.owner_id);
    assert_eq!(doc.name, doc_retrieved.name);
    assert_eq!(doc.doctype, doc_retrieved.doctype);
    assert_eq!(doc.version, doc_retrieved.version);
    assert_eq!(doc.body, doc_retrieved.body);

    Ok(())
}

#[tokio::test]
async fn test_sqlite_engine_cascading_deletes() -> std::io::Result<()> {
    use conduit::doc::document::DigraphDocument;
    use conduit::doc::project::Project;
    use conduit::storage::engine::{Engine, EngineError};

    let _ = env_logger::try_init();
    let storage = Sqlite::setup(":memory:".into())
        .await
        .expect("The sqlite storage to be set up");

    let _ = storage
        .migrate()
        .await
        .expect("The sqlite storage to be migrated");

    let project = Project::new(conduit::util::naming::empty_uuid());
    let project_id = project.id;

    let _ = storage
        .store_project(project.clone())
        .await
        .expect("The project to be inserted");

    let doc = DigraphDocument::create(&project);

    let _ = storage
        .store_document(doc.clone().into())
        .await
        .expect("The document to be inserted");

    let _ = storage
        .delete_project(&project.id)
        .await
        .expect("Project to be deleted from database");

    assert_eq!(
        storage.get_project(&project_id).await,
        Err(EngineError::NotFound)
    );
    assert_eq!(
        storage.get_document(&doc.id).await,
        Err(EngineError::NotFound)
    );

    Ok(())
}
