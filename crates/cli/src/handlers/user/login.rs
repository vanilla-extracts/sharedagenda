use crate::{
    API_URL, TOKEN,
    configuration::loader::{load, write_config},
};
use chrono::{DateTime, Utc};
use common::Answer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginPost<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginAnswer {
    code: i64,
    token: String,
    expiration: Option<DateTime<Utc>>,
}

impl Answer for LoginAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn process_error(&self) {
        println!("Error on login, code {}, message {}", self.code, self.token);
    }

    fn process(&mut self) {
        *TOKEN.lock().unwrap() = self.token.clone();
        let mut config = load().unwrap_or_default();
        config.token = self.token.clone();
        println!("Login is successful");
        match write_config(&config) {
            Ok(_) => {
                println!("Configuration has been updated")
            }
            Err(_) => {
                println!("Error while updating configuration")
            }
        }
    }
}

pub async fn login(vec: Vec<String>) {
    if vec.len() < 2 {
        println!("Usage: login <email> <password>");
        return;
    }

    let data = LoginPost {
        email: &vec[0],
        password: &vec[1],
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<LoginPost<'_>, LoginAnswer>(url, Some(&data), "user", "login").await;
}
