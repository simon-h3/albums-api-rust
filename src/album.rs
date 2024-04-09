use actix_web::{delete, get, post, web, HttpResponse, Responder};
use actix_web::cookie::time::format_description::parse;
use actix_web::web::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, Value};
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
    // pub message: Option<String>,
    pub albumId: u32,
    pub name: String,
    pub tracks: u32,        // Vec<Track>
    pub artists: String,    //Vec<Artist>,
    pub genres: String,     //Vec<Genre>,
    pub year: u16,
    pub rating: u8,
    pub comments: String    //Vec<Comment>
}


// pub type album_db = Response<Vec<Album>>; // get all albums

// GET, get best albums
#[get("/albums")]
pub async fn get_top_albums() -> HttpResponse {
    return HttpResponse::Ok().body("top albums: TBD"); //TODO: implement album structure
}

#[post("/albums")]
async fn create_album(album: web::Json<Album>) -> impl Responder {
    // Process the incoming album data
    let album: Album = album.into_inner();
    save_album_data(&album).await;

    HttpResponse::Created().json(album)
}

async fn save_album_data(album: &Album) {
    // SQLITE CALL...

    println!("Saving album: {:?}", album);
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


