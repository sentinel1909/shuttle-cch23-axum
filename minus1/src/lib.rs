// minus1/src/lib.rs

// dependencies
use actix_web::{get, HttpResponse, Responder};

// index route
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/-1/error")]
async fn error() -> impl Responder {
    HttpResponse::InternalServerError().finish()
}
