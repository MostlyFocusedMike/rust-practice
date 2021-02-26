use postgres::{Client, NoTls, Error as PgError};

#[derive(Debug)]
struct Author {
    id: i32, // postgres int4 converts to this
    name: String,
    age: i32,
    books: Vec<Book>
}

#[derive(Debug)]
struct Book {
    id: i32,
    author_id: i32,
    title: String,
    genre: String,
}


impl Author {
    fn new(client: &mut postgres::Client, name: &str, age: i32) -> Result<Author, PgError> {
        let query = client.query(
            "INSERT INTO authors (name, age) VALUES ($1, $2) RETURNING *",
            &[&name, &age],
        )?;

        let query = query.get(0).expect("new author query broke");
        Ok(Author {
            id: query.get("id"),
            name: query.get("name"),
            age: query.get("age"),
            books: Vec::new(),
        })
    }

    fn get(client: &mut postgres::Client, id: i32) -> Result<Author, PgError> {
        let query = client.query(
            "SELECT * FROM authors JOIN books ON authors.id = books.author_id WHERE authors.id = $1;",
            &[&id],
        )?;

        let mut books = Vec::new();

        for row in &query {
            books.push(Book {
                id: row.get("id"),
                author_id: row.get("author_id"),
                title: row.get("title"),
                genre: row.get("genre"),
            });
        }

        let author_query = query.get(0).expect("get author query broke");

        Ok(Author {
            id: author_query.get("id"),
            name: author_query.get("name"),
            age: author_query.get("age"),
            books,
        })
    }
}

impl Author {
    fn new_book(&self, client: &mut postgres::Client, title: &str, genre: &str) -> Result<Book, PgError> {
        let query = client.query(
            "INSERT INTO books (title, genre, author_id) VALUES ($1, $2, $3) RETURNING *",
            &[&title, &genre, &self.id],
        )?;

        let query = query.get(0).expect("new_book query broke");
        Ok(Book {
            id: query.get("id"),
            title: query.get("title"),
            author_id: query.get("author_id"),
            genre: query.get("genre"),
        })
    }

    fn books(&self, client: &mut postgres::Client) -> Result<Vec<Book>, PgError> {
        let mut books = Vec::new();
        for row in client.query("SELECT * from books WHERE author_id = $1", &[&self.id])? {
            books.push(Book {
                id: row.get("id"),
                author_id: row.get("author_id"),
                title: row.get("title"),
                genre: row.get("genre"),
            });
        }

        Ok(books)
    }
}

fn init(client: &mut postgres::Client) -> Result<(), PgError> {
    client.batch_execute("
    CREATE TABLE IF NOT EXISTS authors (
        id              SERIAL PRIMARY KEY,
        name            VARCHAR NOT NULL,
        age             INT NOT NULL,
        created_at      TIMESTAMP DEFAULT NOW()
    );
    CREATE TABLE IF NOT EXISTS books  (
        id              SERIAL PRIMARY KEY,
        author_id       INTEGER NOT NULL REFERENCES authors,
        title           VARCHAR NOT NULL,
        genre           VARCHAR NOT NULL,
        created_at      TIMESTAMP DEFAULT NOW()
    );")?;

    client.execute("DELETE FROM books", &[])?;
    client.execute("DELETE FROM authors", &[])?;

    Ok(())
}

fn main() -> Result<(), PgError> {
    // This assumes a user/password as "postgres" and a "test" database has already been created
    // This is from the cookbook https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html
    let mut pg_client = Client::connect("postgresql://postgres:postgres@localhost/test", NoTls)?;

    init(&mut pg_client)?;

    let bill = Author::new(&mut pg_client, "bill", 32)?;
    bill.new_book(&mut pg_client, "Terror Tree", "Horror")?;
    bill.new_book(&mut pg_client, "Death Apple", "Horror")?;
    let bill_books = bill.books(&mut pg_client)?;
    println!("{:#?}", bill);
    println!("{:#?}", bill_books);

    let bill_with_books = Author::get(&mut pg_client, bill.id)?;
    println!("{:#?}", bill_with_books);

    Ok(())
}