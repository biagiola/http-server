pub mod rustaceans;
pub mod crates;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);
