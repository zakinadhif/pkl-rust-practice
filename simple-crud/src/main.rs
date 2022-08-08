mod controllers;

use diesel::PgConnection;
use actix_web::{App, HttpServer, web, get};
use goodbye_world::establish_connection;
use controllers::note;

struct AppState {
    db: PgConnection 
}

#[get("/")]
async fn index() -> &'static str {
    "Root of Goodbye World API - A note taking app that is badly named"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db: establish_connection()
            }))
            .service(index)
            .service(
                web::scope("/notes")
                    .service(note::index)
                    .service(note::insert)
                    .service(note::remove)
                    .service(note::update)
                    .service(note::read)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
