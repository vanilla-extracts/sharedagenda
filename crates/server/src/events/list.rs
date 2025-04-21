use chrono::{DateTime, FixedOffset};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    database::Database,
    users::delete::{get_token_struct_from_token, get_uuid_from_token},
};

use super::structs::Event;

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventList<'r> {
    token: &'r str,
    date_start: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventListAnswer {
    code: u16,
    body: String,
    events: Vec<Event>,
}

#[post("/event/list", format = "application/json", data = "<body>")]
pub async fn list(body: Json<EventList<'_>>) -> Json<EventListAnswer> {
    async fn lists(token_uuid: String, date_start: DateTime<FixedOffset>) -> EventListAnswer {
        let token = match get_token_struct_from_token(token_uuid).await {
            Some(t) => t,
            None => {
                return EventListAnswer {
                    code: 402,
                    body: "Token does not exists".to_string(),
                    events: vec![],
                };
            }
        };
        if let Some(owner) = get_uuid_from_token(token).await {
            let db1 = Database::new().await;
            let sql1 = &format!(
                "select id,name,owner,date_start,date_end from events where owner='{}' and date_start > $1",
                owner
            );

            let events: Vec<Event> = db1.query(sql1, &[&date_start]).await;

            EventListAnswer {
                code: 200,
                body: "".to_string(),
                events,
            }
        } else {
            EventListAnswer {
                code: 401,
                body: "Token is expired".to_string(),
                events: vec![],
            }
        }
    }
    let date_start;
    match DateTime::parse_from_str(&body.date_start, "%Y-%m-%d %H:%M %z") {
        Ok(date) => date_start = date,
        Err(e) => {
            println!("{e}");
            return Json(EventListAnswer {
            code: 406,
            body:
                "Error while parsing the start date, the date must be in the following format: %Y-%m-%d %H:%M %z"
                    .to_string(),
            events: vec![]
        });
        }
    }

    Json(lists(body.token.to_string(), date_start).await)
}
