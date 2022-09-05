use actix_files::Files;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_json::json;

use std::io;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
use self::models::*;

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
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let database_url = env::var("DATABASE URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create DB connection pool");

    println!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(Files::new("/static", "static"))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
