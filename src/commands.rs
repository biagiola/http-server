use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString,
        PasswordHasher,
        Error
    },
    Argon2,
};
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::models::NewUser;
use crate::repositories::{RoleRepository, UserRepository};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot retireve DB url from environment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to postres")
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(hashed_password.to_string())
}

pub async fn create_user(
    username: String,
    password: String,
    role_codes: Vec<String>
) {
    let mut c = load_db_connection().await;

    let hashed_password = hash_password(password).unwrap();

    let new_user = NewUser { username, password: hashed_password };
    let user = UserRepository::create(&mut c, new_user, role_codes).await.unwrap();
    println!("User created {:?}", user);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut c).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;

    UserRepository::delete(&mut c, id).await.unwrap();
}
