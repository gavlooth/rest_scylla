use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, middleware::Logger};
use serde::{Serialize};
use scylla::{Session, SessionBuilder, IntoTypedRows};

mod storage;

use crate::storage::scdb::{create_session,initialize_scylladb};


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
async fn init_store_handler(session: &Session) -> impl Responder{
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

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
