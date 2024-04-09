use actix_web::{delete, get, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

use crate::sql_proxy;

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
    pub album_id: u32,
    pub name: String,
    pub tracks: u32, // Vec<Track>
    pub artists: String, //Vec<Artist>,
    pub genres: String, //Vec<Genre>,
    pub year: u16,
    pub rating: u8,
    pub comment: String //Vec<Comment>
}

impl Album{
    fn new(album_id: u32, name: String, tracks: u32, artists: String, genres: String, year: u16, rating :u8, comment: String) -> Album{
        let album: Album = Album {album_id, name, tracks, artists, genres, year, rating, comment};

        return album;
    }
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
    // print!("Creating album: {:?}", album);

    println!("Creating album:\t {:?} :\t {:?}", album.name, Utc::now());

    let album: Album = album.into_inner();  // into itself...    
    save_album_data(&album).await;          // call save_album_data

    println!("Saved album:\t {:?} :\t {:?}\n", album.name, Utc::now());

    HttpResponse::Created().json(album)     
}

async fn save_album_data(album: &Album) {
    // SQLITE CALL...

    sql_proxy::insert_album(album).expect("Failed to insert album");
}

// GET, get album
#[get("/albums/{id}")]
pub async fn get_album(id: web::Path<u32>) -> HttpResponse {
    let album_id = id.into_inner();
    println!("Getting album:\t {} \t: {:?}", album_id, Utc::now());
    
    let album: Album = sql_proxy::get_album_by_id(album_id).await.expect("Failed to get album");
    
    println!("Got album:\t {} \t: {:?}\n", album_id, Utc::now());
    
    HttpResponse::Ok().json(album)
}

// DELETE, delete album
#[delete("/albums/{id}")]
pub async fn delete_album(_id: web::Path<u32>) -> HttpResponse {
    return HttpResponse::Ok().body("delete album: TBD"); //TODO: implement album structure
}


