use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{API_URL, TOKEN, parse_line_into_arguments};

use super::login::{Answer, call};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventCreatePost<'r> {
    token: &'r str,
    date_start: &'r str,
    date_end: &'r str,
    name: &'r str,
    invitees: Vec<&'r str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventCreateAnswer {
    code: i32,
    answer: String,
}

impl Answer for EventCreateAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn answer(&self) -> String {
        self.answer.clone()
    }
    fn process(&mut self) {}
}

pub async fn create(vec: Vec<String>) {
    if vec.len() < 3 {
        println!("Usage: create <name> <date_start> <date_end> [invitees]");
        return;
    }
    let token = TOKEN.lock().unwrap().to_string();
    let mut invitees = vec![];
    if vec.len() > 3 {
        for invitee in vec[3].split(",") {
            invitees.push(invitee);
        }
    }
    let date_start = match NaiveDateTime::parse_from_str(&vec[1], "%Y-%m-%d %H:%M") {
        Ok(e) => e.and_local_timezone(Local::now().fixed_offset().timezone()),
        Err(e) => {
            println!(
                "Error while parsing date, it must be in the following format: %Y-%m-%d %H:%M, {e}"
            );
            return;
        }
    }
    .unwrap()
    .format("%Y-%m-%d %H:%M %z")
    .to_string();

    let date_end = match NaiveDateTime::parse_from_str(&vec[2], "%Y-%m-%d %H:%M") {
        Ok(e) => e.and_local_timezone(Local::now().fixed_offset().timezone()),
        Err(e) => {
            println!(
                "Error while parsing date, it must be in the following format: %Y-%m-%d %H:%M, {e}"
            );
            return;
        }
    }
    .unwrap()
    .format("%Y-%m-%d %H:%M %z")
    .to_string();

    let data = EventCreatePost {
        name: &vec[0],
        token: &token,
        date_start: &date_start,
        date_end: &date_end,
        invitees,
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<EventCreatePost<'_>, EventCreateAnswer>(url, &data, "event", "create").await;
}
