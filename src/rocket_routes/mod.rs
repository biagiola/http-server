use std::error::Error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Value};

pub mod rustaceans;
pub mod crates;

use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    eprintln!("######");
    eprintln!("{}", e);

    if e.to_string() == "Record not found" {
        return Custom(Status::NotFound, json!("Error"))
    }

    Custom(Status::InternalServerError, json!("Error"))
}
