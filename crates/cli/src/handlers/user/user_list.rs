use common::{Call, struct_user::UserListAnswer};

use crate::{API_URL, CliAnswer};

impl CliAnswer for UserListAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn process_error(&self) {
        println!("Error while fetching user list, code {}", self.code);
    }
    fn process(&mut self) {
        self.users.sort_by_key(|f| f.name.clone());
        for user in self.users.clone() {
            println!("------");
            println!("UUID: {}", user.uuid);
            println!("Name: {}", user.name);
            println!("Email: {}", user.email);
            println!("------");
        }
    }
}

pub async fn user_list() {
    let url = API_URL.lock().unwrap().to_string();
    Call::call::<(), UserListAnswer>(url, None, "user", "list").await
}
