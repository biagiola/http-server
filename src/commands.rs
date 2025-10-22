use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::repositories::UserRepository;
use crate::models::*;

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot retireve DB url from environment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to postres")
}

pub async fn create_user(
    username: String,
    password: String,
    roles_codes: Vec<String>
) {
    let mut c = load_db_connection().await;

    let new_user = NewUser { username, password };
    let user = UserRepository::create(&mut c, new_user).await.unwrap();
    println!("User created {:?}", user);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
}
