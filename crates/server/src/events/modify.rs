use chrono::{DateTime, Utc};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{database::Database, users::delete::get_token_struct_from_token};

use super::structs::Event;

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventModification<'r> {
    token: &'r str,
    event_id: i32,
    name: Option<&'r str>,
    date_start: Option<String>,
    date_end: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventModificationAnswer {
    code: u16,
    body: String,
}

pub async fn get_event_from_id_and_owner(id: i32, owner: String) -> Option<Event> {
    let db: Database = Database::new().await;
    db.query(
        &format!(
            "select id,name,owner,date_start,date_end from events where id={} and owner='{}'",
            id, owner
        ),
        &[],
    )
    .await
    .first()
    .cloned()
}

#[post("/event/modify", format = "application/json", data = "<body>")]
pub async fn modify(body: Json<EventModification<'_>>) -> Json<EventModificationAnswer> {
    if let Some(token) = get_token_struct_from_token(body.token.to_string()).await {
        if token.expiration_date < Utc::now() {
            return Json(EventModificationAnswer {
                code: 401,
                body: "Token is expired".to_string(),
            });
        }

        let event = match get_event_from_id_and_owner(body.event_id, token.owner.clone()).await {
            Some(u) => u,
            None => {
                return Json(EventModificationAnswer {
                    code: 408,
                    body: "Event does not exist".to_string(),
                });
            }
        };

        let mut name = event.name;
        let mut date_start = event.date_start;
        let mut date_end = event.date_start;

        if let Some(nm) = body.name {
            name = nm.to_string();
        }

        if let Some(ds) = &body.date_start {
            match DateTime::parse_from_str(ds, "%Y-%m-%d %H:%M %z") {
                Ok(date) => date_start = date,
                Err(e) => {
                    println!("{e}");
                    return Json(EventModificationAnswer {
            code: 406,
            body:
                "Error while parsing the start date, the date must be in the following format: %Y-%m-%d %H:%M %z"
                    .to_string(),
        });
                }
            }
        }

        if let Some(de) = &body.date_end {
            match DateTime::parse_from_str(de, "%Y-%m-%d %H:%M %z") {
                Ok(date) => date_end = date,
                Err(e) => {
                    println!("{e}");
                    return Json(EventModificationAnswer {
            code: 406,
            body:
                "Error while parsing the end date, the date must be in the following format: %Y-%m-%d %H:%M %z"
                    .to_string(),
        });
                }
            }
        }

        let db = Database::new().await;
        let sql = format!(
            "update events set name='{}', date_start=$1, date_end=$2 where owner='{}' and id={}",
            name,
            token.owner.clone(),
            body.event_id
        );
        db.execute(&sql, &[&date_start, &date_end]).await;

        Json(EventModificationAnswer {
            code: 200,
            body: format!("Event {} has been updated", name),
        })
    } else {
        Json(EventModificationAnswer {
            code: 402,
            body: "Token does not exist.".to_string(),
        })
    }
}
