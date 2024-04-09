# Rust API Example with Actix
Demonstrating how to create a RESTful API for managing albums and comments.

## Getting Started

To get started with this project, follow the steps below:

1. Make sure you have Rust and Cargo installed.
2. Clone this repository to your local machine.
3. Install the project dependencies by running:

   `cargo build`

4. Start the server by running:

   `cargo run`

5. The API will be available at http://localhost:8080.

## Endpoints

**Albums**

- GET /albums: *Get best rated albums.*
- POST /albums: *Create a new album.*
- GET /albums/{id}: *Find an album by ID.*
- DELETE /albums/{id}: *Delete an album by ID.*

**Comments**

- GET /albums/{id}/comments: *List comments associated with an album.*
- POST /albums/{id}/comments: *Add a comment to an album.*
- DELETE /albums/{id}/comments/{comment_id}: *Delete a comment from an album.*

**Dependencies**

- Actix-web - A powerful, pragmatic, and extremely fast web framework for Rust.
- Rusqlite - A library that provides a simple way to interface with SQLite databases in Rust.
- Serde - Json shenanigans.

# CURL Examples
`
curl --location 'localhost:8080/albums/0'
`

`
curl --location 'localhost:8080/albums' \
--header 'Content-Type: application/json' \
--data '{
    "album_id": 0,
    "name": "Rust In Peace",
    "tracks": 12,
    "artists": "Megadeth",
    "genres": "Metal",
    "year": 1992,
    "rating": 9,
    "comment": "Thrash masterpiece!"
}'
`