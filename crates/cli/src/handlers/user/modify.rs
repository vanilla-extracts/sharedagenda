use argon2::{Argon2, PasswordHasher};
use common::Answer;
use password_hash::{SaltString, rand_core::OsRng};
use serde::{Deserialize, Serialize};

use crate::{API_URL, TOKEN};

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
    fn process_error(&self) {
        println!(
            "Error while modifying user account, code {}, message {}",
            self.code, self.body
        );
    }
    fn process(&mut self) {
        println!("Your account has been modified succesfully");
    }
}

pub async fn modify(vec: Vec<String>) {
    if vec.len() < 3 {
        println!("Usage: change <name> <email> <password>");
        return;
    }
    let token = TOKEN.lock().unwrap().to_string();

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let password_hashed = match argon.hash_password(vec[2].as_bytes(), &salt) {
        Ok(e) => e.to_string(),
        Err(e) => {
            println!("Error, aborting registration of user.\n{e}");
            return;
        }
    };

    let data = UserModifyPost {
        token: &token,
        name: &vec[0],
        email: &vec[1],
        password: &password_hashed,
    };
    let url = API_URL.lock().unwrap().to_string();
    call::<UserModifyPost<'_>, UserModifyAnswer>(url, Some(&data), "user", "modify").await;
}
