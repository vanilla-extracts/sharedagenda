use rocket::serde::{Deserialize, Serialize, json::Json};
use uuid::Uuid;

use crate::database::Database;

use super::structs::User;

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCreation<'r> {
    name: &'r str,
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCreationAnswer {
    code: i64,
    answer: String,
}

pub async fn check_user_existence(email: &str) -> Option<User> {
    let db = Database::new().await;
    let vec: Vec<User> = db
        .query(&format!("select * from users where email='{email}'"))
        .await;
    if vec.is_empty() {
        None
    } else {
        Some(vec[0].clone())
    }
}

#[post("/user/create", format = "application/json", data = "<body>")]
pub async fn create(body: Json<UserCreation<'_>>) -> Json<UserCreationAnswer> {
    if check_user_existence(body.email).await.is_some() {
        return Json(UserCreationAnswer {
            code: 400,
            answer: "User with the same email exists".to_string(),
        });
    }
    let db = Database::new().await;
    db.execute_statement(&format!(
        "insert into users(uuid,email,name,password) values('{}','{}','{}','{}')",
        Uuid::new_v4(),
        body.email,
        body.name,
        body.password
    ))
    .await;
    Json(UserCreationAnswer {
        code: 200,
        answer: format!("User {} has been created", body.email),
    })
}
