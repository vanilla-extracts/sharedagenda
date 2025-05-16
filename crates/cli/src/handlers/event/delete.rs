use common::Answer;
use serde::{Deserialize, Serialize};

use crate::{API_URL, TOKEN};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventDeletionPost<'r> {
    token: &'r str,
    event_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventDeletionAnswer {
    code: u16,
    body: String,
}

impl Answer for EventDeletionAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn process_error(&self) {
        println!(
            "Error while deleting an event, code {}, message {}",
            self.code, self.body
        );
    }
    fn process(&mut self) {}
}

pub async fn remove(line: &str) {
    let token = TOKEN.lock().unwrap().to_string();
    let event_id: i32 = match line.trim().parse() {
        Ok(id) => id,
        Err(e) => {
            println!("Please specify an integer: {e}");
            return;
        }
    };
    let data = EventDeletionPost {
        token: &token,
        event_id,
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<EventDeletionPost<'_>, EventDeletionAnswer>(url, Some(&data), "event", "delete").await;
}
