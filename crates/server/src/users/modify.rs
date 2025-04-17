use chrono::Utc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::Database;

use super::delete::{get_token_struct_from_token, get_user_from_uuid};

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserModification<'r> {
    token: &'r str,
    password: Option<&'r str>,
    email: Option<&'r str>,
    name: Option<&'r str>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserModificationAnswer {
    code: u16,
    body: String,
}

#[post("/user/modify", format = "application/json", data = "<body>")]
pub async fn modify(body: Json<UserModification<'_>>) -> Json<UserModificationAnswer> {
    if let Some(token) = get_token_struct_from_token(body.token.to_string()).await {
        if token.expiration_date < Utc::now() {
            return Json(UserModificationAnswer {
                code: 401,
                body: "Token is expired".to_string(),
            });
        }

        let user = get_user_from_uuid(token.owner)
            .await
            .expect("User does not exists");
        let mut pass = user.password;
        let mut email = user.email;
        let mut name = user.name;
        if let Some(pswd) = body.password {
            pass = pswd.to_string();
        }
        if let Some(mail) = body.email {
            email = mail.to_string();
        }
        if let Some(nm) = body.name {
            name = nm.to_string();
        }

        let db = Database::new().await;
        let sql = format!(
            "update users set name='{}', email='{}', password='{}' where uuid='{}'",
            name, email, pass, user.uuid
        );
        db.execute_statement(&sql).await;

        Json(UserModificationAnswer {
            code: 200,
            body: format!("User {} has been updated", user.uuid.clone()),
        })
    } else {
        Json(UserModificationAnswer {
            code: 402,
            body: "Token does not exist.".to_string(),
        })
    }
}
