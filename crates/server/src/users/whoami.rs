use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use super::{
    delete::{get_token_struct_from_token, get_user_from_token},
    structs::UserWithoutPassword,
};

extern crate rocket;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserWhoami {
    token: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserWhoamiAnswer {
    code: i32,
    user: Option<UserWithoutPassword>,
}

#[post("/user/whoami", format = "application/json", data = "<body>")]
pub async fn whoami(body: Json<UserWhoami>) -> Json<UserWhoamiAnswer> {
    let token = match get_token_struct_from_token(body.token.clone()).await {
        Some(tk) => tk,
        None => {
            return Json(UserWhoamiAnswer {
                code: 402,
                user: None,
            });
        }
    };
    let user = match get_user_from_token(token).await {
        Some(usr) => usr,
        None => {
            return Json(UserWhoamiAnswer {
                code: 405,
                user: None,
            });
        }
    };
    Json(UserWhoamiAnswer {
        code: 200,
        user: Some(UserWithoutPassword::from_user(&user)),
    })
}
