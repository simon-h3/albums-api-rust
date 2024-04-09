use std::env;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

mod album;
mod comment;
mod sql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    // different 'Result'
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    sql::create_album_table().expect("Failed to create database");

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