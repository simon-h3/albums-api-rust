use actix_web::{get, post, web, delete, HttpResponse};
use actix_web::cookie::time::format_description::parse;
use actix_web::web::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use crate::comment::Comment;

pub struct Genre{
    pub id: u32,
    pub name: String,
}

pub struct Artist{
    pub id: u32,
    pub name: String,
}

pub struct Track{
    pub id: u32,
    pub name: String,
    pub artists: Vec<Artist>,
    pub genres: Vec<Genre>,
    pub length: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Album{
    pub albumId: u32,
    pub name: String,
    pub tracks: u32, // Vec<Track>
    pub artists: String, //Vec<Artist>,
    pub genres: String, //Vec<Genre>,
    pub year: u16,
    pub rating: u8,
    pub comments: String //Vec<Comment>
}
impl Album{
    fn new(albumId: u32, name: String, tracks: u32, artists: String, genres: String, year: u16, rating :u8, comments: String) -> Album{
        let album: Album = Album {albumId, name, tracks, artists, genres, year, rating, comments};

        return album;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumRequest {
    pub message: Option<String>,
}

impl AlbumRequest {
    pub fn to_album(&self) -> Option<Album> {
        match &self.message {
            Some(message) => {
                let parsed_message = from_str(&message)?;
                Some(Album::new(parsed_message["albumId"], parsed_message["name"], parsed_message["tracks"], parsed_message["artist"], parsed_message["genre"], parsed_message["year"], parsed_message["rating"], parsed_message["comment"]))
            }
            None => None,
            }
    }
}

// pub type album_db = Response<Vec<Album>>; // get all albums

// GET, get best albums
#[get("/albums")]
pub async fn get_top_albums() -> HttpResponse {
    return HttpResponse::Ok().body("top albums: TBD"); //TODO: implement album structure
}

// POST, create album
#[post("/albums")]
pub async fn create_album(album_req: Json<AlbumRequest>) -> HttpResponse {
    return HttpResponse::Created().json(album_req.to_album()); //TODO: implement album structure
}

// GET, get album
#[get("/albums/{id}")]
pub async fn get_album(id: web::Path<u32>) -> HttpResponse {
    return HttpResponse::Ok().body("get album: TBD"); //TODO: implement album structure
}

// DELETE, delete album
#[delete("/albums/{id}")]
pub async fn delete_album(id: web::Path<u32>) -> HttpResponse {
    return HttpResponse::Ok().body("delete album: TBD"); //TODO: implement album structure
}


