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

    if e.to_string() == "Record not found" {
        return Custom(Status::NotFound, json!("Error"))
    }

    Custom(Status::InternalServerError, json!("Error"))
}

// #[rocket::get("/rustaceans/<id>")]
// pub async fn view_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
//     RustaceanRepository::find(&mut db, id).await
//         .map(|rustacean| json!(rustacean))
//         .map_err(|e| match e {
//             diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
//             _ => server_error(e.into())
//         })
// }
