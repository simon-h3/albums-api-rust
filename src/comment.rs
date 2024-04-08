use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};

pub struct Comment{
    pub user_id: i32,
    pub time_created: DateTime<Utc>,
    pub text: String,

}

// GET, list comments from album ID
#[get("/albums/{id}/comments")]
pub async fn get_comments() -> HttpResponse {
    return HttpResponse::Ok().body("get comments: TBD"); //TODO: implement comment data structure
}

// POST, create new comment
#[post("/albums/{id}/comments")]
pub async fn add_comment() -> HttpResponse {
    return HttpResponse::Ok().body("add comment: TBD"); //TODO: implement comment data structure
}

// DELETE, delete comment
#[delete("/albums/{id}/comments/{comment_id}")]
pub async fn delete_comment() -> HttpResponse {
    return HttpResponse::Ok().body("delete comment: TBD"); //TODO: implement comment data structure
}