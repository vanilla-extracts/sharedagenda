use common::{Answer, call};

use crate::{
    API_URL, TOKEN,
    configuration::loader::{load, write_config},
    structs::struct_user::{LogoutAnswer, LogoutPost},
};

impl Answer for LogoutAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn process_error(&self) {
        println!("Error while logging out, code {}", self.code);
    }
    fn process(&mut self) {
        *TOKEN.lock().unwrap() = "".to_string();

        println!("You successfully logged out");

        let mut config = load().unwrap_or_default();
        config.token = "".to_string();

        if write_config(&config).is_err() {
            println!("Error while updating configuration");
        }
    }
}

pub async fn logout() {
    let token = TOKEN.lock().unwrap().to_string();
    let data = LogoutPost { token: &token };
    let url = API_URL.lock().unwrap().to_string();
    call::<LogoutPost<'_>, LogoutAnswer>(url, Some(&data), "user", "logout").await;
}
