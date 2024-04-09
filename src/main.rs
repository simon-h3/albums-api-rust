use std::env;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
// use std::io::Result;
use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;

mod album;
mod comment;

fn db_test() -> std::io::Result<()> {
    let conn = Connection::open("albums.db")?;

    conn.execute("CREATE TABLE IF NOT EXISTS Albums (albumId INTEGER NOT NULL, name TEXT NOT NULL,
        tracks INTEGER, artist TEXT, genre TEXT, year INTEGER, rating INTEGER, comment TEXT,
        PRIMARY KEY (albumId))", ())?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {    // different 'Result'
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    db_test()?;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(album::get_top_albums)
            .service(album::get_album)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}