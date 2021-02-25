use postgres::{Client, NoTls, Error as PgError};
// use std::collections::HashMap;

#[derive(Debug)]
struct Author {
    id: i32, // postgres int4 converts to this for some reason,
    name: String,
    age: i32,
    genre: String,
}

impl Author {
    fn create(pg_client: &mut postgres::Client, name: &str, genre: &str, age: i32) -> Result<Author, PgError> {
        let query = pg_client.query(
            "INSERT INTO authors (name, genre, age) VALUES ($1, $2, $3) RETURNING *",
            &[&name, &genre, &age],
        )?;
        let query = query.get(0).unwrap();

        Ok(Author {
            id: query.get("id"),
            name: query.get("name"),
            age: query.get("age"),
            genre: query.get("genre"),
        })
    }
}


fn main() -> Result<(), PgError> {
    // This assumes a user/password as "postgres" and a "test" database has already been created
    // This is from the cookbook https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/test", NoTls)?;

    // They use batch execute, but you don't seem to need to for tables
    client.batch_execute("CREATE TABLE IF NOT EXISTS authors (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            genre           VARCHAR NOT NULL,
            age             INT NOT NULL
        )
    ")?;

    // created_at      TIMESTAMP DEFAULT NOW()
    client.execute("CREATE TABLE IF NOT EXISTS books  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES authors
        )
    ", &[])?;

    client.execute("DELETE FROM authors", &[])?;

    let bill = Author::create(&mut client, "bill", "Horror", 32).unwrap();
    let sara = Author::create(&mut client, "sara", "Comedy", 45).unwrap();
    let bo = Author::create(&mut client, "bo", "Action", 84).unwrap();
    println!("{:#?}", bill);
    println!("{:#?}", sara);
    println!("{:#?}", bo);


    // let thing: i32 = bill.get(0).unwrap().get("id");
    // println!("t {:#?}", thing);

    // for row in bill {
    //     let author = Author {
    //         id: row.get("id"),
    //         name: row.get(1),
    //         genre: row.get(2),
    //         age: row.get(3),
    //     };
    //     println!("{:#?}", author);
    // }
    // // let authors: HashMap<_, _> = author_seed.into_iter().collect();

    // for row in client.query("SELECT id, name, country FROM author", &[])? {
    //     let author = Author {
    //         id: row.get(0),
    //         name: row.get(1),
    //         country: row.get(2),
    //     };
    //     println!("Author {} is from {}, id is {}", author.name, author.country, author.id);
    // }

    Ok(())
}