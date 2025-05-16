use common::{
    Answer, Call,
    struct_event::{EventDeletionAnswer, EventDeletionPost},
};

use crate::{API_URL, CliAnswer, TOKEN};

impl CliAnswer for EventDeletionAnswer {
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
    Call::call::<EventDeletionPost<'_>, EventDeletionAnswer>(url, Some(&data), "event", "delete")
        .await;
}
