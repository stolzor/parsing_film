extern crate yaml_rust;
use postgres::{Client, NoTls};

mod database;
mod status;
mod utils;
mod parser;

fn main() -> Result<(), postgres::Error> {
    // Init config from dev.yaml
    let config = utils::get_config();
    let db_url = config["DB_URL"].as_str().unwrap();

    // Init client
    let mut client = Client::connect(db_url, NoTls)?;

    // Init table
    let init_table = match database::films::init_table(& mut client) {
        status::QueryStatus::Complete => 200,
        status::QueryStatus::Error => panic!("Some kind of error when initializing the table")
    };
    println!("Status {:?}", init_table);

    // Parser module
    parser::start_parsing();

    Ok(())
}
