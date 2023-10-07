use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};

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

async fn not_found(  ) -> Result<HttpResponse>
{
    let response = Response {
        message: "Resourse not found".to_string()

    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
                    .service(healthcheck)
                    .default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
