use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, middleware::Logger};
use serde::{Serialize};
use scylla::{Session};
use std::sync::Arc;
use actix_files::{NamedFile, Files};
use std::path::PathBuf;
mod storage;

use crate::storage::scdb::{initialize_scylladb, create_session, create_scylladb};

#[derive(Clone)]
struct AppState{
db :  Arc<Session>
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
async fn init_store_handler(state:  web::Data<AppState>) -> impl  Responder{
    let conn = & state.db;

    create_scylladb(conn).await.unwrap();
    initialize_scylladb(conn).await.unwrap();
    let response = Response {
     message: "db initialized".to_string()
    };
    HttpResponse::Ok().json(response)

}

// #[get("/")]
// async fn index(_req: HttpRequest) -> Result<NamedFile> {
//     let path: PathBuf = "./static/index.html".parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }
//

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
    match  create_session(&db_string).await {
        Ok(v) =>
        {
            let state = web::Data::new( AppState{db : v.into()} );
            HttpServer::new(move || {
                let logger = Logger::default();
                App::new()
                    .wrap(logger)
                    .app_data(state.clone())
                    .service(healthcheck)
                    .service(init_store_handler)
                    .service(Files::new("/", "./static").show_files_listing())
                    .default_service(web::route().to(not_found))
            })
            .bind(("127.0.0.1", 9090))?
                .run()
                .await
        },

        Err(e) => panic!("Problem opening the file: {:?}", e),


    }

}

// use actix_files::fs;
// App::new()
//        .route("/", web::get().to(index))
//        .route("/again", web::get().to(index2))
//       .service(fs::Files::new("/app", "./main_page").show_files_listing())
