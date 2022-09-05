use actix_files::Files;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_json::json;

use std::io;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{connection, prelude::*};

mod models;
mod schema;
use self::models::*;
use self::schema::cats::dsl::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

struct IndexTemplatedata {
    project_name: String,
    cats: Vec<self::models::Cat>,
}

async fn index(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("can't get db connection from pool");

    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .map_err(|| HttpResponse::InternalServerError().finish())?;

    let data = IndexTemplatedata {
        project_name: "Catdex".to_string(),
        cats: cats_data,
    };

    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

async fn add(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {
    let body = hb.render("add", &{}).unwrap();
    Ok(HttpResponse::Ok().body(body))
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
            .route("/add", web::get().to(add))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
