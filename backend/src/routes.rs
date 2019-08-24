use actix_web::{HttpResponse, HttpRequest};
use actix_web::web::{Data, Form};
use std::sync::Arc;
use mongodb::ClientInner;
use actix_web::http::StatusCode;

pub fn notFound(req:HttpRequest) ->HttpResponse{
    HttpResponse::NotFound().content_type("text/html").body(include_str!("../../frontend/404.html"))
}