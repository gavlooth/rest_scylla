use actix_web::{HttpServer,  middleware::Logger, get, post, App, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body);
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!");
}


#[actix_web::main]

async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug") ;
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(hello)
            .service (echo)
            .route("/hey",web::get(.to(manual_hello)))
    })
    .bind(("127.0.0.1", 9080))?
        .run()
        .await


        // println!("Hello, world!");
}
