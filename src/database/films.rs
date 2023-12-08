use postgres::Client;
use crate::status::QueryStatus;

pub fn init_table(client: & mut Client) -> QueryStatus {
    let query = "
        CREATE TABLE person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ";

    let result = match client.batch_execute(query) {
        Ok(()) => QueryStatus::Complete,
        Err(error) => {
            let error = error.as_db_error().unwrap();
            if error.code().code() == "42P07" {
                return QueryStatus::Complete;
            } else {
                return  QueryStatus::Error;
            }
        },
    };

    result
}

// pub fn query_create_films(client: &Client) {
// }
