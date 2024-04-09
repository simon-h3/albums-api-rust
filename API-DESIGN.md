## REST API Example with Actix

/albums

 - **GET**: Get best rated albums.
 - **POST**: Create new album.

/albums/:id

 - **GET**: Find an album by ID.
 - **DELETE**: Delete an album by ID.

/albums/:id/comments

 - **GET**: List comments associated with an album.
 - **POST**: Add a comment to an album.
 - **DELETE**: Delete comment from an album.

# SQLite Data

**Album:**
    albumId      - integer (u32)
    name    - String  (TEXT)
    tracks  - integer (u32)
    artist  - String  (TEXT)
    genre   - String  (TEXT)
    year    - integer (u16)
    rating  - integer (u8)
    comment - String  (TEXT) 

`
CREATE TABLE IF NOT EXISTS Albums (albumId INTEGER NOT NULL, name TEXT NOT NULL, tracks INTEGER, artist TEXT, genre TEXT, 
    year INTEGER, rating INTEGER, comment TEXT, PRIMARY KEY (albumId))
`

JSON:
`
{
    "albumId": 0,
    "name": "Rust In Peace",
    "tracks": 12,
    "artist": "Megadeth",
    "genre": "Metal",
    "year": "1992",
    "rating": 9,
    "comment": "Thrash masterpiece!"
}
`
# CURL Examples


