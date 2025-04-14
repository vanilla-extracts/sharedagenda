extern crate rocket;

#[get("/user/modify")]
pub fn modify() -> &'static str {
    "Hello, World!"
}
