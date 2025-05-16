use common::structs::struct_event::{WhoamiAnswer, WhoamiPost};

use crate::{
    API_URL, TOKEN,
    call::{Answer, call},
};

impl Answer for WhoamiAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn process_error(&self) {
        println!("Error while fetching user information, code {}", self.code);
    }
    fn process(&mut self) {
        match &self.user {
            Some(usr) => {
                println!("---- User Information ----");
                println!("UUID: {}", usr.uuid);
                println!("Name: {}", usr.name);
                println!("Email: {}", usr.email);
                println!("---- User Information ----");
            }
            None => {
                println!("No user found");
            }
        }
    }
}

pub async fn whoami() {
    let token = TOKEN.lock().unwrap().to_string();
    let data = WhoamiPost { token: &token };
    let url = API_URL.lock().unwrap().to_string();
    call::<WhoamiPost<'_>, WhoamiAnswer>(url, Some(&data), "user", "whoami").await;
}
