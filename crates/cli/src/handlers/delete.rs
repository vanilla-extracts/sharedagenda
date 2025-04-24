use serde::{Deserialize, Serialize};

use crate::{
    API_URL, TOKEN,
    configuration::loader::{load, write_config},
};

use super::login::{Answer, call};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeletePost<'r> {
    token: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteAnswer {
    code: u16,
    body: String,
}

impl Answer for DeleteAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn answer(&self) -> String {
        *TOKEN.lock().unwrap() = "".to_string();

        *TOKEN.lock().unwrap() = "".to_string();

        let mut config = load().unwrap_or_default();
        config.token = "".to_string();

        match write_config(&config) {
            Ok(_) => {
                format!("{}", self.body)
            }
            Err(_) => {
                format!("Error while updating configuration")
            }
        }
    }
}

pub async fn delete() {
    let token = TOKEN.lock().unwrap().to_string();
    let data = DeletePost { token: &token };
    let url = API_URL.lock().unwrap().to_string();
    call::<DeletePost<'_>, DeleteAnswer>(url, &data, "user", "delete").await;
}
