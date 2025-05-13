use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::Database;

use super::structs::UserWithoutPassword;

extern crate rocket;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserListAnswer {
    code: i32,
    users: Vec<UserWithoutPassword>,
}

async fn get_list_of_users() -> Vec<UserWithoutPassword> {
    let db = Database::new().await;
    let result: Vec<UserWithoutPassword> = db.query("select uuid,email,name from users", &[]).await;
    result
}

#[get("/user/list")]
pub async fn list() -> Json<UserListAnswer> {
    Json(UserListAnswer {
        code: 200,
        users: get_list_of_users().await,
    })
}
