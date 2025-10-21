mod models;
mod schema;
mod repositories;
mod rocket_routes;

use rocket_routes::rustaceans::{
    get_rustaceans,
    view_rustacean,
    create_rustacean,
    update_rustacean,
    delete_rustacean,
};
use rocket_routes::crates::{
    get_crates,
    view_crate,
    create_crate,
    update_crate,
    delete_crate,
};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean,
            get_crates,
            view_crate,
            create_crate,
            update_crate,
            delete_crate,
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
