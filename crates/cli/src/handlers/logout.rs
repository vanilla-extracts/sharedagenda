use serde::{Deserialize, Serialize};

use crate::{
    API_URL, TOKEN,
    configuration::loader::{load, write_config},
};

use super::login::{Answer, call};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogoutPost<'r> {
    token: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogoutAnswer {
    code: i32,
}

impl Answer for LogoutAnswer {
    fn code(&self) -> i32 {
        self.code
    }
    fn answer(&self) -> String {
        if self.code != 200 {
            "Error while login in".to_string()
        } else {
            "You have successfully logout".to_string()
        }
    }
    fn process(&mut self) {
        *TOKEN.lock().unwrap() = "".to_string();

        let mut config = load().unwrap_or_default();
        config.token = "".to_string();

        if let Err(_) = write_config(&config) {
            println!("Error while updating configuration");
        }
    }
}

pub async fn logout() {
    let token = TOKEN.lock().unwrap().to_string();
    let data = LogoutPost { token: &token };
    let url = API_URL.lock().unwrap().to_string();
    call::<LogoutPost<'_>, LogoutAnswer>(url, &data, "user", "logout").await;
}
