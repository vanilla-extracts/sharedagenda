use chrono::{Local, NaiveDateTime};
use common::structs::struct_event::{EventModifyAnswer, EventModifyPost};

use crate::{
    API_URL, TOKEN,
    call::{Answer, call},
};

impl Answer for EventModifyAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn process_error(&self) {
        println!(
            "Error while modifiying an event, code {}, message {}",
            self.code, self.body
        );
    }
    fn process(&mut self) {
        println!("The event has been modified successfully.")
    }
}

pub async fn change(vec: Vec<String>) {
    if vec.len() < 4 {
        println!("Usage: change <id> <name> <date_start> <date_end>");
        return;
    }
    let token = TOKEN.lock().unwrap().to_string();
    let event_id: i32 = match vec[0].parse() {
        Ok(e) => e,
        Err(_) => {
            println!("Error while parsing event id, please input a valid integer");
            return;
        }
    };

    let date_start = match vec[2].trim() {
        "" => "".to_string(),
        _ => match NaiveDateTime::parse_from_str(&vec[2], "%Y-%m-%d %H:%M") {
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
        .to_string()
    };

    let date_end = match vec[3].trim() {
        "" => "".to_string(),
        _ => match NaiveDateTime::parse_from_str(&vec[3], "%Y-%m-%d %H:%M") {
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
        .to_string()
    };

    let data = EventModifyPost {
        name: if vec[1].trim() == "" {
            None
        } else {
            Some(&vec[1])
        },
        token: &token,
        date_start: if date_start.trim() == "" {
            None
        } else {
            Some(&date_start)
        },
        date_end: if date_end.trim() == "" {
            None
        } else {
            Some(&date_end)
        },
        event_id,
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<EventModifyPost<'_>, EventModifyAnswer>(url, Some(&data), "event", "modify").await;
}
