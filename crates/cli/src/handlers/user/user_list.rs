use common::structs::struct_user::UserListAnswer;

use crate::{
    API_URL,
    call::{Answer, call},
};

impl Answer for UserListAnswer {
    fn code(&self) -> i32 {
        self.code as i32
    }
    fn process_error(&self) {
        eprintln!("Error while fetching user list, code {}", self.code);
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
    call::<(), UserListAnswer>(url, None, "user", "list").await
}
