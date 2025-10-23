use crate::rocket_routes::{CacheConn, DbConn, server_error};
use crate::repositories::UserRepository;
use crate::auth::{Credentials, authorize_user};
use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use rocket::response::status::Custom;

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, mut cache: Connection<CacheConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut db, &credentials.username).await
        .map_err(|e| server_error(e.into()))?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_|Custom(Status::Unauthorized, json!("Unauthorized")))?;

    // insert into redis. TODO: implement redis repository
    cache.set_ex::<String, i32, usize>(
        format!("sessions/{}", session_id),
        user.id,
        3*60*60
    ).await
    .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id,
    }))
}
