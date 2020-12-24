use clap::{App, load_yaml, crate_version, crate_authors};

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

