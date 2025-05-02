use std::fmt::Debug;

use argon2::{Argon2, PasswordHasher};
use chrono::{DateTime, Utc};
use password_hash::{SaltString, rand_core::OsRng};
use reqwest::Client;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    API_URL, TOKEN,
    configuration::loader::{load, write_config},
    parse_line_into_arguments,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginPost<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginAnswer {
    status: i64,
    token: String,
    expiration: Option<DateTime<Utc>>,
}

pub trait Answer {
    fn code(&self) -> i32;
    fn answer(&self) -> String;
}

impl Answer for LoginAnswer {
    fn code(&self) -> i32 {
        self.status as i32
    }
    fn answer(&self) -> String {
        if self.status == 200 {
            format!(
                "Login is successfull, token is {} which expires at {}",
                self.token,
                self.expiration.unwrap()
            )
        } else {
            format!("{}", self.token)
        }
    }
}

pub async fn call<U: Serialize + Debug, V: DeserializeOwned + Answer>(
    url: String,
    data: &U,
    first_route: &str,
    second_route: &str,
) -> Option<V> {
    let client = Client::new();
    match client
        .post(format!("{}/{}/{}", url, first_route, second_route))
        .json(data)
        .send()
        .await
    {
        Ok(e) => {
            // println!("{}", e.text().await.unwrap());
            // None
            match e.json::<V>().await {
                Ok(answer) => {
                    if answer.code() != 200 {
                        println!(
                            "Error while sending the request \nCode: {} \nMessage: {} ",
                            answer.code(),
                            answer.answer()
                        );
                        Some(answer)
                    } else {
                        println!("{}", answer.answer());
                        Some(answer)
                    }
                }
                Err(e) => {
                    println!("Error while matching the answer: {e}");
                    None
                }
            }
        }
        Err(e) => {
            println!("Error while sending the resquest: {e}");
            None
        }
    }
}

pub async fn login(line: &str) {
    let vec = parse_line_into_arguments(line);
    if vec.len() < 2 {
        println!("Usage: login <email> <password>");
        return;
    }

    let data = LoginPost {
        email: &vec[0],
        password: &vec[1],
    };
    let url = API_URL.lock().unwrap().to_string();
    let log = call::<LoginPost<'_>, LoginAnswer>(url, &data, "user", "login").await;
    if let Some(answer) = log {
        *TOKEN.lock().unwrap() = answer.clone().token;
        let mut config = load().unwrap_or_default();
        config.token = answer.clone().token;

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
