use clap::{crate_authors, crate_version, load_yaml, App};
use std::fs::File;
use std::path::Path;
#[macro_use]
extern crate serde;

pub mod doc;
pub mod model;
pub mod storage;

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
        if !Path::new(&url).exists() {
            println!("db: file does not exist, creating it");
            let _ = File::create(&url)?;
        }
        if let Some(ref _matches) = matches.subcommand_matches("migrate") {
            match crate::storage::sqlite::Sqlite::setup(url).await {
                Ok(db) => {
                    println!("db: running migrations");
                    db.migrate().await?
                }
                Err(err) => {
                    println!("Error {}", err);
                }
            }
        }
    }

    Ok(())
}
