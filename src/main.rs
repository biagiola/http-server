use rocket_db_pools::{Connection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use rocket::serde::json::Value;
use crate::repositories::RustaceanRepository;

mod models;
mod schema;
mod repositories;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 100).await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            get_rustaceans
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
