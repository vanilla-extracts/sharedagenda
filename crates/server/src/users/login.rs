use chrono::{DateTime, Utc};
use rocket::{
    serde::{Deserialize, Serialize, json::Json},
    tokio::spawn,
};

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
    let token = Token::new(owner);
    async fn insert_token(token: Token) {
        let db = Database::new().await;
        let sql = &format!(
            "insert into token(token,owner,expiration_date) values('{}','{}',$1)",
            token.token, token.owner
        );
        db.execute(sql, &[&token.expiration_date]).await;
    }
    spawn(insert_token(token.clone()));
    token
}

async fn delete_token(owner: String) {
    let db = Database::new().await;
    let sql = &format!("delete from token where owner = '{}'", owner);
    db.execute_statement(sql).await;
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
        if old_token.expiration_date < Utc::now() {
            spawn(delete_token(owner.clone()));
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
                    status: 404,
                    token: "Password does not match".to_string(),
                    expiration: None,
                })
            }
        }
        None => Json(UserLoginAnswer {
            status: 405,
            token: "User does not exist".to_string(),
            expiration: None,
        }),
    }
}
