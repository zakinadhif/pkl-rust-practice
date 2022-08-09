mod controllers;

use goodbye_world::establish_connection;
use controllers::note;

use diesel::PgConnection;
use actix_web::{App, HttpServer, web, get};
use dotenv::dotenv;

use std::env;

pub struct AppState {
    db: PgConnection
}

#[get("/")]
async fn index() -> &'static str {
    "Root of Goodbye World API - A note taking app that is badly named"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse()
        .expect("PORT env variable must be an integer");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db: establish_connection()
            }))
            .service(index)
            .service(
                web::scope("/notes")
                    .route("", web::get().to(note::index))
                    .route("", web::post().to(note::insert))
                    .route("/{note_id}", web::delete().to(note::remove))
                    .route("/{note_id}", web::put().to(note::update))
                    .route("/{note_id}", web::get().to(note::read))
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
