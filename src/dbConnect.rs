use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Establish a connection to the SQLite database
    let conn = Connection::open("rssqlitedemo.sqlite")?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
        )",
        [], // No parameters.
    )?;

    // Insert a new person into the table
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        params!["Alice", 30],
    )?;

    // Query the database and print out the results
    let mut stmt = conn.prepare("SELECT id, name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }

    Ok(())
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}