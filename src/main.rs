use std::env;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

mod album;
mod comment;
mod sql_proxy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    // different 'Result'
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    println!("Starting server...");

    sql_proxy::delete_album_table().expect("Failed to delete database");
    sql_proxy::create_album_table().expect("Failed to create database");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(album::get_top_albums)
            .service(album::get_album)
            .service(album::create_album)
            .service(album::delete_album)
            .service(comment::get_comments)
            .service(comment::add_comment)
            .service(comment::delete_comment)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}