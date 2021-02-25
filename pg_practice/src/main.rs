use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

struct Author {
    id: i32,
    name: String,
    country: String
}


fn main() -> Result<(), Error> {
    // This assumes a user/password as "postgres" and a "library" database has already been created
    // This is from the cookbook https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

    // They use batch execute, but you don't seem to need to for tables
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
        )
    ")?;

    client.execute("
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ", &[])?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    client.execute("DELETE FROM author", &[])?;

    for (key, value) in &authors {
        let author = Author {
            id: 0,
            name: key.to_string(),
            country: value.to_string()
        };

        client.execute(
                "INSERT INTO author (name, country) VALUES ($1, $2)",
                &[&author.name, &author.country],
        )?;
    }

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}, id is {}", author.name, author.country, author.id);
    }

    Ok(())
}