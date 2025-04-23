use reqwest::Client;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::API_URL;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginPost<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginAnswer {
    code: i32,
    token: String,
    expiration: String,
}

pub trait Answer {
    fn code(&self) -> i32;
    fn answer(&self) -> String;
}

impl Answer for LoginAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn answer(&self) -> String {
        format!("Token {}", self.token)
    }
}

pub async fn call<U: Serialize, V: DeserializeOwned + Answer>(
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
            let answer: V = e.json().await.unwrap();
            if answer.code() != 200 {
                println!(
                    "Error while sending the request \n Code: {} \n Message: {} ",
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
            println!("Error while sending the resquest: {e}");
            None
        }
    }
}

pub async fn login(line: &str) {
    let args = line.split_whitespace();
    let mut vec = vec![];
    for arg in args {
        vec.push(arg);
    }
    if vec.len() < 2 {
        println!("Usage: login <email> <password>");
        return;
    }
    let data = LoginPost {
        email: vec[0],
        password: vec[1],
    };
    let url = API_URL.with(|f| f.take());
    let log = call::<LoginPost<'_>, LoginAnswer>(url, &data, "user", "login").await;
    match log {
        Some(answer) => println!("{}", answer.token),
        None => println!("Error"),
    }
}
