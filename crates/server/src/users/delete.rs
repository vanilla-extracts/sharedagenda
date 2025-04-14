extern crate rocket;

#[get("/user/delete")]
pub fn delete() -> &'static str {
    "Hello, World!"
}
