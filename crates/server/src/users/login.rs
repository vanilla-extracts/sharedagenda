use std::i64;

use rocket::serde::{Deserialize, Serialize, json::Json};
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLoginAnswer {
    status: i64,
    token: String,
}

#[post("/user/login", format = "application/json", data = "<body>")]
pub async fn login(body: Json<UserLogin<'_>>) -> Json<UserLoginAnswer> {
    Json(UserLoginAnswer {
        status: 200,
        token: "".to_string(),
    })
}
