use std::env;
use diesel::Connection;
use diesel::pg::PgConnection;

pub mod models;
pub mod schema;


pub fn get_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
