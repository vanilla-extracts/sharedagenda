use rocket::serde::{Deserialize, Serialize, json::Json};

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCreation<'r> {
    name: &'r str,
    email: &'r str,
    password: &'r str,
}

fn check_user_existence(email: &str) -> bool {
    false
}

#[post("/user/create", format = "application/json", data = "<body>")]
pub fn create(body: Json<UserCreation<'_>>) -> &'static str {
    "Hello World"
}
