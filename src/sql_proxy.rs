use rusqlite::{Connection, Result};
use crate::album::Album;

pub fn create_album_table() -> Result<()> {
    let conn = Connection::open("albums.db")?;

    conn.execute("CREATE TABLE IF NOT EXISTS Albums (albumId INTEGER NOT NULL, name TEXT NOT NULL,
        tracks INTEGER, artist TEXT, genre TEXT, year INTEGER, rating INTEGER, comment TEXT,
        PRIMARY KEY (albumId))", ())?;

    Ok(())
}

pub fn insert_album(album: &Album) -> Result<()> {
    let conn = Connection::open("albums.db")?;

    let stmt: String = format!("INSERT INTO Albums (albumId, name, tracks, artist, genre, year, rating, comment) 
        VALUES ({}, '{}', {}, '{}', '{}', {}, {}, '{}')",
        album.album_id, album.name, album.tracks, album.artists, album.genres, album.year, album.rating, album.comment);

    conn.execute(&stmt, ())?;   // NO PARAMS...

    Ok(())
}