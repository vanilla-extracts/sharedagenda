use users::{creation::create, delete::delete, login::login, logout::logout, modify::modify};

#[macro_use]
extern crate rocket;

mod events;
mod users;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create, login, logout, modify, delete])
}
