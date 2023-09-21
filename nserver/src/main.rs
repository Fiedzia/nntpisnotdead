use actix_web::{error, get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use bb8;
use bb8_postgres;
use derive_more;
use diesel;
use log::{info};
use num_cpus;
use std::env;

mod config;
mod db;
mod utils;

struct AppData {
    config: config::Config,
    db_pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<bb8_postgres::tokio_postgres::NoTls>>,
}


#[derive(Debug, derive_more::Display, derive_more::Error)]
enum MyServerError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,
}

impl error::ResponseError for MyServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyServerError::BadClientData => StatusCode::BAD_REQUEST,
        }
    }
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/a")]
async fn helloa(app_data: web::Data<AppData>) -> Result<HttpResponse, MyServerError> {
    let mut client = app_data.db_pool.get().await.unwrap();
    let res = client
        .query("select 1", &[])
        .await
        .map_err(|e| MyServerError::InternalError)?;
    let data: Vec<_> = res.iter().map(|r| { let x: i32 = r.get(0); x}).collect();
    Ok(HttpResponse::Ok().body(format!("Hello from {}, {:?}", app_data.config.server.host, data)))
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

    let manager = bb8_postgres::PostgresConnectionManager::new(
        env::var("DATABASE_URL").unwrap().parse().unwrap(),
        bb8_postgres::tokio_postgres::NoTls,
    );
    let db_pool = bb8::Pool::builder()
        .max_size(match config.db.pool_size {
            0 => num_cpus::get() as u32,
            v => v
        })
        .build(manager)
        .await
        .expect("Failed to initiate db pool ");


    let (host, port) = (config.server.host.clone(), config.server.port.clone());
    info!("Starting server");
    HttpServer::new(move || {

        App::new()
            .app_data(web::Data::new(AppData {
                config: config.clone(),
                db_pool: db_pool.clone(),
            }))
            .service(hello)
            .service(helloa)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port))?
    .run()
    .await
}
