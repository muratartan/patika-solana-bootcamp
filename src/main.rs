use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("listening on port 8080");
    HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
