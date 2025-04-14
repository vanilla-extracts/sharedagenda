#[macro_use]
extern crate rocket;

mod events;
mod users;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![users::creation::index])
}
