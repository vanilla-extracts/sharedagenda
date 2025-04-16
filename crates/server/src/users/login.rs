use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize, json::Json};

use crate::database::Database;

use super::{creation::check_user_existence, structs::Token};
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
    expiration: Option<DateTime<Utc>>,
}

async fn create_and_insert_new_token(owner: String) -> Token {
    let db = Database::new().await;
    let token = Token::new(owner);
    db.execute(
        &format!(
            "insert into token(token,owner,expiration_date) values('{}','{}',$1)",
            token.token, token.owner
        ),
        &[&token.expiration_date],
    )
    .await;
    token
}

pub async fn create_or_take_token(owner: String) -> Token {
    let db = Database::new().await;
    let vec: Vec<Token> = db
        .query(&format!(
            "select token,owner,expiration_date from token where owner='{}'",
            owner
        ))
        .await;
    if vec.is_empty() {
        create_and_insert_new_token(owner).await
    } else {
        let old_token = vec[0].clone();
        let new_connection = Database::new().await;
        if old_token.expiration_date < Utc::now() {
            new_connection
                .execute_statement(&format!("delete from token where owner='{}'", owner))
                .await;
            create_and_insert_new_token(owner).await
        } else {
            old_token
        }
    }
}

#[post("/user/login", format = "application/json", data = "<body>")]
pub async fn login(body: Json<UserLogin<'_>>) -> Json<UserLoginAnswer> {
    match check_user_existence(body.email).await {
        Some(user) => {
            if user.password == body.password {
                let token = create_or_take_token(user.uuid).await;
                Json(UserLoginAnswer {
                    status: 200,
                    token: token.token,
                    expiration: Some(token.expiration_date),
                })
            } else {
                Json(UserLoginAnswer {
                    status: 400,
                    token: "Password does not match".to_string(),
                    expiration: None,
                })
            }
        }
        None => Json(UserLoginAnswer {
            status: 400,
            token: "User does not exist".to_string(),
            expiration: None,
        }),
    }
}
