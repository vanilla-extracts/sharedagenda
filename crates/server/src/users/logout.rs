use rocket::{http::Status, serde::json::Json, tokio::spawn};
use serde::{Deserialize, Serialize};

use crate::database::Database;

extern crate rocket;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserLogout {
    token: String,
}

async fn delete_token(token: String) {
    let db = Database::new().await;
    db.execute_statement(&format!("delete from token where token='{}'", token))
        .await;
}

#[post("/user/logout", format = "application/json", data = "<body>")]
pub async fn logout(body: Json<UserLogout>) -> Status {
    spawn(delete_token(body.token.clone()));
    Status { code: 200 }
}
