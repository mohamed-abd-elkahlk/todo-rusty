use rusqlite::{params, Connection, Result};

fn db() -> Result<()> {
    // Open a connection to the database, or create one if it doesn't exist
    let conn = Connection::open("todo.db")?;

    // Create a new table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}
