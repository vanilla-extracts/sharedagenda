use chrono::Utc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::Database;

use super::{
    create::get_user_from_email,
    delete::{get_token_struct_from_token, get_user_from_uuid},
};

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

        let user = match get_user_from_uuid(token.owner).await {
            Some(u) => u,
            None => {
                return Json(UserModificationAnswer {
                    code: 408,
                    body: "Event does not exist".to_string(),
                });
            }
        };

        let mut pass = user.password;
        let mut email = user.email;
        let mut name = user.name;
        if let Some(pswd) = body.password {
            if pswd.trim() != "" {
                pass = pswd.trim().to_string();
            }
        }
        if let Some(mail) = body.email {
            if mail.trim() != "" && get_user_from_email(mail.trim()).await.is_none() {
                email = mail.trim().to_string();
            }
        }
        if let Some(nm) = body.name {
            if nm.trim() != "" {
                name = nm.trim().to_string();
            }
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
