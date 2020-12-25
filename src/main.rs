use clap::{crate_authors, crate_version, load_yaml, App};
#[macro_use]
extern crate serde;

pub mod model;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    if let Some(input) = matches.value_of("INPUT") {
        println!("{}", input);
    } else {
        println!("No input file given");
    }
}
