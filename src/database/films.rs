use postgres::Client;
use crate::{status::QueryStatus, parser::FilmFields};

pub fn init_table(client: & mut Client) -> QueryStatus {
    let query = "
        CREATE TABLE films (
            id SERIAL PRIMARY KEY,
            page INT NOT NULL,
            title_name TEXT NOT NULL,
            title_href TEXT NOT NULL,
            mail_rate TEXT NULL,
            idmb_rate TEXT NULL
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

pub fn query_create_films(client: & mut Client, film: &FilmFields) -> QueryStatus {
    let query =
        "
        INSERT INTO films (page, title_name, title_href, mail_rate, idmb_rate)
        VALUES ($1, $2, $3, $4, $5)
        ";
    
    let result = match client.execute(
        query,
        &[&film.page, &film.title_name, &film.title_href, &film.mail_rate, &film.imdb_rate]
    ) {
        Err(_) => QueryStatus::Error,
        Ok(_) => QueryStatus::Complete
    };

    result
}

pub fn query_get_films(client: & mut Client, film: &FilmFields) -> QueryStatus {
    let query = 
    "
        SELECT * FROM films WHERE title_href like $1
    ";

    let result = match client.execute(
        query,
        &[&film.title_href]
    ) {
        Err(_) => QueryStatus::Error,
        Ok(s) => {
            if s == 0 {
                return QueryStatus::Error;
            }
            QueryStatus::Complete
        }
    };

    result
}
