extern crate rocket;

#[get("/user/login")]
pub fn login() -> &'static str {
    "Hello, World!"
}
