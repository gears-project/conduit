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
