use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::API_URL;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterPost<'r> {
    name: &'r str,
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterAnswer {
    code: i32,
    answer: String,
}

async fn call(url: String, data: &RegisterPost<'_>) {
    let client = Client::new();
    match client
        .post(format!("{}/user/create", url))
        .json(data)
        .send()
        .await
    {
        Ok(e) => {
            let answer: RegisterAnswer = e.json().await.unwrap();
            if answer.code != 200 {
                println!(
                    "Error while sending the request \n Code: {} \n Message: {} ",
                    answer.code, answer.answer
                );
            } else {
                println!("{}", answer.answer);
            }
        }
        Err(e) => {
            println!("Error while sending the resquest: {e}")
        }
    }
}

pub async fn register(line: &str) {
    let args = line.split_whitespace();
    let mut vec = vec![];
    for arg in args {
        vec.push(arg);
    }
    if vec.len() < 3 {
        println!("Usage: register <name> <email> <password>");
        return;
    }
    let data = RegisterPost {
        name: vec[0],
        email: vec[1],
        password: vec[2],
    };
    let url = API_URL.with(|f| f.take());
    call(url, &data).await;
}
