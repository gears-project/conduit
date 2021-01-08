use clap::{crate_authors, crate_version, load_yaml, App};

extern crate conduit;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("db") {
        println!("Running command 'db'");
        let url = "test.db".to_string();
        if let Some(ref _matches) = matches.subcommand_matches("migrate") {
            match conduit::storage::sqlite::Sqlite::setup(url).await {
                Ok(db) => {
                    println!("db: running migrations");
                    db.migrate().await?
                }
                Err(err) => {
                    println!("Error {}", err);
                }
            }
        }
    } else if let Some(ref _matches) = matches.subcommand_matches("serve") {
        let _ = conduit::http::server::serve();
    }

    Ok(())
}
