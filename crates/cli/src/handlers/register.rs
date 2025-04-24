use serde::{Deserialize, Serialize};

use crate::API_URL;

use super::login::{Answer, call};

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

impl Answer for RegisterAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn answer(&self) -> String {
        self.answer.clone()
    }
}

pub async fn register(line: &str) {
    let args = line.split("%");
    let mut vec = vec![];
    for arg in args {
        vec.push(arg.trim());
    }
    if vec.len() < 3 {
        println!("Usage: register <name>%<email>%<password>");
        return;
    }
    let data = RegisterPost {
        name: vec[0],
        email: vec[1],
        password: vec[2],
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<RegisterPost<'_>, RegisterAnswer>(url, &data, "user", "create").await;
}
