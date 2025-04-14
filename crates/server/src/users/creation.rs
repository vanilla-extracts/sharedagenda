extern crate rocket;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, World!"
}
