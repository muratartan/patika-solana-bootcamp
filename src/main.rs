use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;

use std::io;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name": "British short hair",
                "image_path": "/static/image/british-shorthair.jpg"
            },
            {
                "name": "British long hair",
                "image_path": "/static/image/british-longhair.jpg"
            },
            {
                "name": "Russian blue",
                "image_path": "/static/image/russian-blue.jpg"
            },
            {
                "name": "Van cat",
                "image_path": "/static/image/van-cat.jpg"
            },
        ]
    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Listening on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "static"))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
