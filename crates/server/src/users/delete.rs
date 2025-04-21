use chrono::Utc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::Database;

use super::structs::{Token, User};

extern crate rocket;

pub async fn get_user_from_uuid(uuid: String) -> Option<User> {
    let db = Database::new().await;
    let user: Vec<User> = db
        .query(&format!("select * from users where uuid='{}'", uuid), &[])
        .await;
    user.first().cloned()
}

pub async fn get_uuid_from_token(token: Token) -> Option<String> {
    if token.expiration_date < Utc::now() {
        return None;
    }
    Some(token.owner)
}

pub async fn get_user_from_token(token: Token) -> Option<User> {
    match get_uuid_from_token(token).await {
        None => None,
        Some(uuid) => get_user_from_uuid(uuid).await,
    }
}

pub async fn get_token_struct_from_token(token: String) -> Option<Token> {
    let db = Database::new().await;
    db.query(
        &format!(
            "select token,owner,expiration_date from token where token='{}'",
            token
        ),
        &[],
    )
    .await
    .first()
    .cloned()
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDeletion<'r> {
    token: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDeletionAnswer {
    code: u16,
    body: String,
}

#[post("/user/delete", format = "application/json", data = "<body>")]
pub async fn delete(body: Json<UserDeletion<'_>>) -> Json<UserDeletionAnswer> {
    async fn deletion(token_uuid: String) -> UserDeletionAnswer {
        let token = match get_token_struct_from_token(token_uuid).await {
            Some(t) => t,
            None => {
                return UserDeletionAnswer {
                    code: 402,
                    body: "Token does not exists".to_string(),
                };
            }
        };
        if let Some(uuid) = get_uuid_from_token(token).await {
            let db1 = Database::new().await;
            let sql1 = &format!("delete from token where owner='{}'", uuid);
            db1.execute_statement(sql1).await;
            let db = Database::new().await;
            let sql = &format!("delete from users where uuid='{}'", uuid);
            db.execute_statement(sql).await;
            UserDeletionAnswer {
                code: 200,
                body: "User has been successfully deleted".to_string(),
            }
        } else {
            UserDeletionAnswer {
                code: 401,
                body: "Token is expired".to_string(),
            }
        }
    }
    Json(deletion(body.token.to_string()).await)
}
