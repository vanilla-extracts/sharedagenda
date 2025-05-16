use argon2::{Argon2, PasswordHasher};
use common::Answer;
use password_hash::{SaltString, rand_core::OsRng};
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

impl Answer for RegisterAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn process_error(&self) {
        println!(
            "Error while registering, code {}, message {}",
            self.code, self.answer
        );
    }
    fn process(&mut self) {
        println!("Your account has been created successfully, you can now log in");
    }
}

pub async fn register(vec: Vec<String>) {
    if vec.len() < 3 {
        println!("Usage: register <name> <email> <password>");
        return;
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let password_hashed = match argon.hash_password(vec[2].as_bytes(), &salt) {
        Ok(e) => e.to_string(),
        Err(e) => {
            println!("Error, aborting registration of user.\n{e}");
            return;
        }
    };

    let data = RegisterPost {
        name: &vec[0],
        email: &vec[1],
        password: &password_hashed,
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<RegisterPost<'_>, RegisterAnswer>(url, Some(&data), "user", "create").await;
}
