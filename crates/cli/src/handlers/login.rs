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
    fn answer(&self) -> String;
    fn process(&mut self);
}

impl Answer for LoginAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn answer(&self) -> String {
        if self.code == 200 {
            format!(
                "Login is successfull, token is {} which expires at {}",
                self.token,
                self.expiration.unwrap()
            )
        } else {
            self.token.to_string()
        }
    }

    fn process(&mut self) {
        *TOKEN.lock().unwrap() = self.token.clone();
        let mut config = load().unwrap_or_default();
        config.token = self.token.clone();

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
    data: &U,
    first_route: &str,
    second_route: &str,
) {
    let client = Client::new();
    match client
        .post(format!("{}/{}/{}", url, first_route, second_route))
        .json(data)
        .send()
        .await
    {
        Ok(e) => match e.json::<V>().await {
            Ok(mut answer) => {
                if answer.code() != 200 {
                    println!(
                        "Error while sending the request \nCode: {} \nMessage: {} ",
                        answer.code(),
                        answer.answer()
                    );
                } else {
                    println!("{}", answer.answer());
                    answer.process();
                }
            }
            Err(e) => {
                println!("Error while matching the answer: {e}");
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
    call::<LoginPost<'_>, LoginAnswer>(url, &data, "user", "login").await;
}
