use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{API_URL, TOKEN};

use super::login::{Answer, call};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhoamiPost<'r> {
    token: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhoamiAnswer {
    code: i64,
    user: Option<User>,
}

impl Answer for WhoamiAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn answer(&self) -> String {
        if self.code == 200 {
            String::new()
        } else {
            format!("{}", self.code)
        }
    }
    fn process(&mut self) {
        match &self.user {
            Some(usr) => {
                println!("---- User Information ----");
                println!("UUID: {}", usr.uuid);
                println!("Name: {}", usr.name);
                println!("Email: {}", usr.email);
                println!("Password: {}", usr.password);
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
