extern crate rocket;

#[get("/user/logout")]
pub fn logout() -> &'static str {
    "Hello, World!"
}
