use serde::{Deserialize, Serialize};

use crate::{API_URL, TOKEN};

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
        *TOKEN.lock().unwrap() = "".to_string();
        format!("You have been successfully log out.")
    }
}

pub async fn logout() {
    let token = TOKEN.lock().unwrap().to_string();
    let data = LogoutPost { token: &token };
    let url = API_URL.lock().unwrap().to_string();
    call::<LogoutPost<'_>, LogoutAnswer>(url, &data, "user", "logout").await;
}
