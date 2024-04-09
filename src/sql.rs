use rusqlite::{Connection, Result};

pub fn create_album_table() -> Result<()> {
    let conn = Connection::open("albums.db")?;

    conn.execute("CREATE TABLE IF NOT EXISTS Albums (albumId INTEGER NOT NULL, name TEXT NOT NULL,
        tracks INTEGER, artist TEXT, genre TEXT, year INTEGER, rating INTEGER, comment TEXT,
        PRIMARY KEY (albumId))", ())?;

    Ok(())
}