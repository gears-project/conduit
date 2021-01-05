use tempdir::TempDir;

extern crate conduit;
use conduit::storage::sqlite::Sqlite;

#[async_std::test]
async fn test_sqlite_in_memory() -> std::io::Result<()> {
    let _ = env_logger::try_init();
    let storage = Sqlite::setup(":memory:".into())
        .await
        .expect("The sqlite storage to be set up");
    assert_eq!(
        storage
            .migrate()
            .await
            .expect("The sqlite storage to be migrated"),
        ()
    );

    Ok(())
}

#[async_std::test]
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
    assert_eq!(storage.migrate().await.expect("Migrations to be run"), ());

    Ok(())
}

#[async_std::test]
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
        .expect("The document to be inserted");

    let mut doc = DigraphDocument::default();
    doc.project_id = project.id;

    let raw_doc: RawDocument = doc.clone().into();

    let _ = storage
        .store_document(raw_doc.clone())
        .await
        .expect("The document to be inserted");

    let retrieved_raw_doc = storage
        .get_document(doc.id)
        .await
        .expect("The stored document to be retrieved");

    assert_eq!(raw_doc, retrieved_raw_doc);

    let doc_retrieved: DigraphDocument = retrieved_raw_doc.into();

    assert_eq!(doc, doc_retrieved);

    Ok(())
}
