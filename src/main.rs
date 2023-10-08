use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, middleware::Logger};
use serde::{Serialize};
use scylla::{Session, SessionBuilder, IntoTypedRows};
use std::sync::Mutex;
mod storage;

use crate::storage::scdb::{create_session,initialize_scylladb};

#[derive(Clone)]
struct AppState<'a> {
db : &'a Session
}

#[derive(Serialize)]
pub struct Response {
    pub message: String

}
#[get("/health")]
async fn healthcheck( ) -> impl Responder{
    let response = Response {
     message: "Health check OK".to_string()
    };
    HttpResponse::Ok().json(response)

}

#[post("/api/initialize_store")]
async fn init_store_handler(state:  web::Data< AppState <'_ > >) -> impl  Responder{
    let conn = state.db;
    initialize_scylladb(conn).await ;
    let response = Response {
     message: "Health check OK".to_string()
    };
    HttpResponse::Ok().json(response)

}

async fn not_found(  ) -> Result<HttpResponse>
{
    let response = Response {
        message: "Resourse not found".to_string()

    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug") ;
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let db_string = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    // let state = AppState{db: & create_session(&db_string).await.unwrap()};


    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            // .app_data(state.clone())
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
