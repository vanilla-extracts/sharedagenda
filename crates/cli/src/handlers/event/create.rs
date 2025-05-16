use chrono::{Local, NaiveDateTime};
use common::{
    Answer, Call,
    struct_event::{EventCreateAnswer, EventCreatePost},
};

use crate::{API_URL, CliAnswer, TOKEN};

impl CliAnswer for EventCreateAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn process_error(&self) {
        println!(
            "Error while creating the event, code {}, message {}",
            self.code, self.body
        );
    }
    fn process(&mut self) {
        println!("The event has been created successfully.");
    }
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
    Call::call::<EventCreatePost<'_>, EventCreateAnswer>(url, Some(&data), "event", "create").await;
}
