use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::database::Database;

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogout<'r> {
    token: &'r str,
}

async fn delete_token(token: &str) {
    let db = Database::new().await;
    db.execute_statement(&format!("delete from token where token='{}'", token))
        .await;
}

#[post("/user/logout", format = "application/json", data = "<body>")]
pub async fn logout(body: Json<UserLogout<'_>>) -> Status {
    delete_token(body.token).await;
    Status { code: 200 }
}
