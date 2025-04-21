use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    database::Database,
    users::delete::{get_token_struct_from_token, get_uuid_from_token},
};

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventDeletion<'r> {
    token: &'r str,
    event_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventDeletionAnswer {
    code: u16,
    body: String,
}

#[post("/event/delete", format = "application/json", data = "<body>")]
pub async fn delete(body: Json<EventDeletion<'_>>) -> Json<EventDeletionAnswer> {
    async fn deletion(token_uuid: String, event_id: i32) -> EventDeletionAnswer {
        let token = match get_token_struct_from_token(token_uuid).await {
            Some(t) => t,
            None => {
                return EventDeletionAnswer {
                    code: 402,
                    body: "Token does not exists".to_string(),
                };
            }
        };
        if let Some(owner) = get_uuid_from_token(token).await {
            let db1 = Database::new().await;
            let sql1 = &format!(
                "delete from events where id={} and owner='{}'",
                event_id, owner
            );
            db1.execute_statement(sql1).await;
            EventDeletionAnswer {
                code: 200,
                body: format!("Event {} has been successfully deleted", event_id),
            }
        } else {
            EventDeletionAnswer {
                code: 401,
                body: "Token is expired".to_string(),
            }
        }
    }
    Json(deletion(body.token.to_string(), body.event_id).await)
}
