use argon2::{Argon2, PasswordHasher};
use common::{
    Answer, Call,
    struct_user::{UserModifyAnswer, UserModifyPost},
};
use password_hash::{SaltString, rand_core::OsRng};

use crate::{API_URL, CliAnswer, TOKEN};

impl CliAnswer for UserModifyAnswer {
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
    Call::call::<UserModifyPost<'_>, UserModifyAnswer>(url, Some(&data), "user", "modify").await;
}
