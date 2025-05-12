use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{API_URL, TOKEN};

use super::login::{Answer, call};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListPost<'r> {
    token: &'r str,
    date_start: String,
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
    code: u16,
    body: String,
    events: Vec<Event>,
}

impl Answer for ListAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn answer(&self) -> String {
        self.body.clone()
    }
    fn process(&mut self) {
        self.events.sort_by_key(|f| f.date_start);
        for event in self.events.clone() {
            println!("------");
            println!("Event: {}", event.id);
            println!("Name: {}", event.name);
            println!("Start: {}", event.date_start.format("%Y-%m-%d %H:%M %z"));
            println!("End: {}", event.date_end.format("%Y-%m-%d %H:%M %z"));
            println!("------");
        }
    }
}

pub async fn list(line: String) {
    let token = TOKEN.lock().unwrap().to_string();
    let mut date = line.trim().to_string().clone();
    if line.trim() == "" {
        date = Local::now().format("%Y-%m-%d %H:%M %z").to_string();
    } else {
        date = match NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M") {
            Ok(e) => e.and_local_timezone(Local::now().fixed_offset().timezone()),
            Err(e) => {
                println!("Error while parsing time, format must be %Y-%m-%d %H:%M, {e}");
                return;
            }
        }
        .unwrap()
        .format("%Y-%m-%d %H:%M %z")
        .to_string();
    }
    let data = ListPost {
        token: &token,
        date_start: date,
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<ListPost<'_>, ListAnswer>(url, Some(&data), "event", "list").await
}
