use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize, json::Json};

use crate::{
    database::Database,
    users::{
        create::get_user_from_email,
        delete::{get_token_struct_from_token, get_user_from_token},
        structs::User,
    },
};

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventCreation<'r> {
    token: &'r str,
    name: &'r str,
    invitees: Vec<String>,
    date_start: String,
    date_end: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventCreationAnswer {
    code: i64,
    body: String,
}

#[post("/event/create", format = "application/json", data = "<body>")]
pub async fn create(body: Json<EventCreation<'_>>) -> Json<EventCreationAnswer> {
    if let Some(token) = get_token_struct_from_token(body.token.to_string()).await {
        if token.expiration_date < Utc::now() {
            return Json(EventCreationAnswer {
                code: 401,
                body: "Token is expired".to_string(),
            });
        }

        let user = match get_user_from_token(token).await {
            Some(usr) => usr,
            None => {
                return Json(EventCreationAnswer {
                    code: 405,
                    body: "User does not exists".to_string(),
                });
            }
        };

        let date_start = match DateTime::parse_from_str(&body.date_start, "%Y-%m-%d %H:%M %z") {
            Ok(date) => date,
            Err(e) => {
                println!("{e}");
                return Json(EventCreationAnswer
 {
            code: 406,
            body:
                "Error while parsing the start date, the date must be in the following format: %Y-%m-%d %H:%M %z"
                    .to_string(),
        });
            }
        };

        let date_end = match DateTime::parse_from_str(&body.date_end, "%Y-%m-%d %H:%M %z") {
            Ok(date) => date,
            Err(e) => {
                println!("{e}");
                return Json(EventCreationAnswer
 {
            code: 406,
            body:
                "Error while parsing the end date, the date must be in the following format: %Y-%m-%d %H:%M %z"
                    .to_string(),
        });
            }
        };

        if date_end < date_start {
            return Json(EventCreationAnswer {
                code: 407,
                body: "Error, the end of the event must be after the start of the event"
                    .to_string(),
            });
        }

        let mut invitees: Vec<User> = Vec::new();
        invitees.push(user.clone());
        for invitee in body.invitees.clone() {
            if let Some(user) = get_user_from_email(&invitee).await {
                invitees.push(user);
            }
        }
        for usr in invitees {
            let db = Database::new().await;
            db.execute(
                &format!(
                    "insert into events(name,date_start,date_end,owner) values('{}',$1,$2,'{}')",
                    body.name, usr.uuid
                ),
                &[&date_start, &date_end],
            )
            .await;
        }

        return Json(EventCreationAnswer {
            code: 200,
            body: format!("Event {} has been created.", body.name),
        });
    }
    Json(EventCreationAnswer {
        code: 402,
        body: "Token does not exists".to_string(),
    })
}
