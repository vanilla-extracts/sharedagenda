use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::Database;

use super::structs::User;

extern crate rocket;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserListAnswer {
    code: i32,
    user: Vec<User>,
}

async fn get_list_of_users() -> Vec<User> {
    let db = Database::new().await;
    let result: Vec<User> = db
        .query(&format!("select uuid,email,name,password from users"), &[])
        .await;
    result
}

#[get("/user/list")]
pub async fn list() -> Json<UserListAnswer> {
    Json(UserListAnswer {
        code: 200,
        user: get_list_of_users().await,
    })
}
