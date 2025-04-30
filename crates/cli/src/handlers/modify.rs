use serde::{Deserialize, Serialize};

use crate::{API_URL, TOKEN, parse_line_into_arguments};

use super::login::{Answer, call};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserModifyPost<'r> {
    token: &'r str,
    name: &'r str,
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserModifyAnswer {
    code: i32,
    body: String,
}

impl Answer for UserModifyAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn answer(&self) -> String {
        self.body.clone()
    }
}

pub async fn modify(line: &str) {
    let vec = parse_line_into_arguments(line);
    if vec.len() < 3 {
        println!("Usage: change <name> <email> <password>");
        return;
    }
    let token = TOKEN.lock().unwrap().to_string();
    let data = UserModifyPost {
        token: &token,
        name: &vec[0],
        email: &vec[1],
        password: &vec[2],
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<UserModifyPost<'_>, UserModifyAnswer>(url, &data, "user", "modify").await;
}
