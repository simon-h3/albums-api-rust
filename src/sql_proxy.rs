use rusqlite::{params, Connection, Result};
use crate::album::{self, Album};

/*
    Get Top Albums (Rank Ratings)

    SELECT * FROM Albums SORT Rating Descending LIMIT 10

*/

pub const PATH: &str = "albums.db";

pub fn delete_album_table() -> Result<()> {
    let conn = Connection::open(PATH)?;

    conn.execute("DROP TABLE IF EXISTS Albums", ())?;

    Ok(())
}


pub fn create_album_table() -> Result<()> {
    let conn = Connection::open(PATH)?;

    conn.execute("CREATE TABLE IF NOT EXISTS Albums (album_id INTEGER NOT NULL, name TEXT NOT NULL,
        tracks INTEGER, artist TEXT, genre TEXT, year INTEGER, rating INTEGER, comment TEXT,
        PRIMARY KEY (album_id))", ())?;

    Ok(())
}

pub fn insert_album(album: &Album) -> Result<()> {
    let conn = Connection::open(PATH)?;

    let stmt: String = format!("INSERT INTO Albums (album_id, name, tracks, artist, genre, year, rating, comment) 
        VALUES ({}, '{}', {}, '{}', '{}', {}, {}, '{}')",
        album.album_id, album.name, album.tracks, album.artists, album.genres, album.year, album.rating, album.comment);

    conn.execute(&stmt, ())?;   // NO PARAMS...

    Ok(())
}

pub async fn get_album_by_id(album_id: u32) -> Result<Album> {

    let conn: Connection = Connection::open(PATH)?;

    let mut stmt = conn.prepare("SELECT * FROM albums WHERE album_id = ?")?;
    
    let album = stmt.query_row(params![album_id], |row| {
        Ok(Album {
            album_id: row.get(0)?,
            name: row.get(1)?,
            tracks: row.get(2)?,
            artists: row.get(3)?,
            genres: row.get(4)?,
            year: row.get(5)?,
            rating: row.get(6)?,
            comment: row.get(7)?,
        })
    })?;

    Ok(album)
}

pub async fn get_top_albums() -> Result<Vec<Album>> {
    let albums: Vec<Album> = vec![]; // assign empty

    let conn: Connection = Connection::open()

    Ok(albums)
}