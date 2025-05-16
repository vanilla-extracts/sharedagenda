use std::fmt::Debug;

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    API_URL, TOKEN,
    configuration::loader::{load, write_config},
};

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

pub trait Answer {
    fn code(&self) -> i32;
    fn process_error(&self);
    fn process(&mut self);
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

pub async fn call<U: Serialize + Debug, V: DeserializeOwned + Answer>(
    url: String,
    data: Option<&U>,
    first_route: &str,
    second_route: &str,
) {
    let client = match data {
        Some(js) => Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(format!("{}/{}/{}", url, first_route, second_route))
            .json(js)
            .send(),
        None => Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(format!("{}/{}/{}", url, first_route, second_route))
            .send(),
    };
    match client.await {
        Ok(e) => match e.json::<V>().await {
            Ok(mut answer) => {
                if answer.code() != 200 {
                    answer.process_error();
                } else {
                    answer.process();
                }
            }
            Err(e) => {
                println!("Error while deserializing answer: {e}");
            }
        },
        Err(e) => {
            println!("Error while sending the resquest: {e}");
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
