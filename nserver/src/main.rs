use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{info};

mod config;
mod db;
mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    utils::read_env_file_if_present()?;
    env_logger::init();
    let config = config::load_config_from_file("config.toml")
        .expect("Error reading config file");
    info!("Starting server");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
