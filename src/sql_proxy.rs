use rusqlite::{params, Connection, Result};
use crate::album::Album;

pub const PATH: &str = "albums.db";

pub const CREATE_ALBUM_TABLE: &str = "CREATE TABLE Albums (\
                                    album_id INTEGER NOT NULL,\
                                    name TEXT NOT NULL,\
                                    year INTEGER,\
                                    rating INTEGER,\
                                    comment TEXT,\
                                    PRIMARY KEY (album_id)\
                                    )";

pub const CREATE_TRACK_TABLE: &str = "CREATE TABLE Tracks (track_id INTEGER NOT NULL,\
                                    album_id INTEGER REFERENCES Albums(album_id),\
                                    name TEXT NOT NULL,\
                                    duration INTEGER, PRIMARY KEY (track_id))"; // UNIX TIME (SQLITE doesn't support 'TIME' type)

pub const CREATE_GENRES_TABLE: &str = "CREATE TABLE Genres (genre_id INTEGER NOT NULL,\
                                    name TEXT NOT NULL,\
                                    PRIMARY KEY (genre_id)\
                                    )";

pub const CREATE_COMMENTS_TABLE: &str = "CREATE TABLE Comments (comment_id TEXT NOT NULL,\
                                        album_id INTEGER REFERENCES Albums(album_id),\
                                        comment TEXT NOT NULL,\
                                        created_at INTEGER,\
                                        PRIMARY KEY (comment_id)\
                                        )";

pub const ALBUM_ARTIST: &str = "CREATE TABLE Album_Artist (\
                                album_id INTEGER REFERENCES Albums(album_id),\
                                artist_id INTEGER REFERENCES Artists(artist_id),\
                                PRIMARY KEY (album_id, artist_id)\
                                )";

pub const ALBUM_GENRES: &str = "CREATE TABLE Album_Genres (
                                album_id INTEGER REFERENCES Albums(album_id),
                                genre_id INTEGER REFERENCES Genres(genre_id),
                                PRIMARY KEY (album_id, genre_id)
                                );";

pub const ALBUM_TRACKS: &str = "CREATE TABLE Album_Tracks (
                                album_id INTEGER REFERENCES Albums(album_id),
                                track_id INTEGER REFERENCES Tracks(track_id),
                                PRIMARY KEY (album_id, track_id)
                                );";

/*

TODO: Modify sql inserts.

Example Inserts

-- Insert an album
INSERT INTO Albums (name, year, rating, comment) VALUES ('Album 1', 2022, 8, 'Great album!');

-- Insert a track
INSERT INTO Tracks (album_id, name, duration) VALUES (1, 'Track 1', '00:03:45');

-- Insert an artist
INSERT INTO Artists (name) VALUES ('Artist 1');

-- Link artist to album
INSERT INTO Album_Artists (album_id, artist_id) VALUES (1, 1);

-- Insert a genre
INSERT INTO Genres (name) VALUES ('Rock');

-- Link genre to album
INSERT INTO Album_Genres (album_id, genre_id) VALUES (1, 1);

-- Insert a comment
INSERT INTO Comments (album_id, comment) VALUES (1, 'Loved this track!');


 */

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

pub fn construct_database() -> Result<()>{


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
    let mut albums: Vec<Album> = vec![]; // assign empty

    let conn: Connection = Connection::open(PATH)?;

    let mut stmt = conn.prepare("SELECT * FROM Albums ORDER BY rating DESC LIMIT 3;")?;

    let albums_iter = stmt.query_map([], |row| {
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

    for album in albums_iter {
        albums.push(album?);
    }

    Ok(albums)
}