extern crate rocket;

#[get("/user/create")]
pub fn create() -> &'static str {
    "Hello, World!"
}
