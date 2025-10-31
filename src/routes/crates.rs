use crate::models::{Crate, NewCrate, User};
use crate::repositories::CrateRepository;
use crate::routes::{DbConn, EditorUser, server_error};
use rocket_db_pools::Connection;
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, json, Value};

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100).await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id).await
        .map(|a_crate| json!(a_crate))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCrate>, _user: EditorUser) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner()).await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", format="json", data="<crates>")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, crates: Json<Crate>, _user: EditorUser) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, crates.into_inner()).await
        .map(|a_crate| json!(a_crate))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32, _user: EditorUser) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}

// site note:
// fn into(self) -> Box<dyn Error, Global>
