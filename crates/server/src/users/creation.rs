use rocket::serde::{Deserialize, Serialize, json::Json};

extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCreation<'r> {
    name: &'r str,
    email: &'r str,
    password: &'r str,
}

#[post("/user/create", data = "<body>")]
pub fn create(body: Json<UserCreation<'_>>) -> &'static str {
    "Hello, World!"
}
