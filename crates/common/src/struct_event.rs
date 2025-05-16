use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventCreatePost<'r> {
    pub token: &'r str,
    pub date_start: &'r str,
    pub date_end: &'r str,
    pub name: &'r str,
    pub invitees: Vec<&'r str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventCreateAnswer {
    pub code: i32,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventDeletionPost<'r> {
    pub token: &'r str,
    pub event_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventDeletionAnswer {
    pub code: u16,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListPost<'r> {
    pub token: &'r str,
    pub date_start: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub date_start: DateTime<FixedOffset>,
    pub date_end: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListAnswer {
    pub code: u16,
    pub body: String,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeletePost<'r> {
    pub token: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteAnswer {
    pub code: u16,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhoamiPost<'r> {
    pub token: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhoamiAnswer {
    pub code: i64,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventModifyPost<'r> {
    pub token: &'r str,
    pub event_id: i32,
    pub date_start: Option<&'r str>,
    pub date_end: Option<&'r str>,
    pub name: Option<&'r str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventModifyAnswer {
    pub code: i32,
    pub body: String,
}
