use crate::routes::{CacheConn, DbConn, server_error};
use crate::repositories::{UserRepository, RedisRepository};
use crate::auth::{Credentials, authorize_user};
use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::response::status::Custom;

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, mut cache: Connection<CacheConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut db, &credentials.username).await
        .map_err(|e| server_error(e.into()))?;

    let user = match user {
        Some(user) => user,
        None => return Err(Custom(Status::Unauthorized, json!("Invalid username or password")))
    };

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_|Custom(Status::Unauthorized, json!("Invalid username or password")))?;

    // Create session in Redis with 3 hour TTL
    RedisRepository::create_session(&mut cache, session_id.clone(), user.id, 3*60*60).await
        .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id,
    }))
}
