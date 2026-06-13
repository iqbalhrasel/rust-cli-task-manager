use rusqlite::{Connection, Result};

pub fn init() -> Result<Connection> {
    let conn = Connection::open("app.db")?;

    create_tables(&conn)?;

    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
         CREATE TABLE IF NOT EXISTS tasks (
             id INTEGER,
             description TEXT NOT NULL,
             done boolean NOT NULL DEFAULT false,
             PRIMARY KEY(id)
         );
         ",
    )?;

    Ok(())
}
