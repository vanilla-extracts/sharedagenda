use std::process::exit;

use configuration::{load, write_default_config};
use database::Database;

#[macro_use]
extern crate rocket;

pub mod configuration;
pub mod database;
mod events;
pub mod structs;
mod users;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello World"
}

#[launch]
async fn rocket() -> _ {
    let configuration = match load() {
        Ok(config) => config,
        Err(_) => match write_default_config() {
            Ok(()) => load().unwrap(),
            Err(e) => {
                println!("Fatal error cannot create file {}", e);
                std::process::exit(1);
            }
        },
    };

    let database = Database::new().await;
    match database.setup_database().await {
        Ok(_) => println!("Tables have been successfully created."),
        Err(e) => {
            println!("{e}");
            exit(1)
        }
    };

    let figment = rocket::Config::figment()
        .merge(("port", configuration.listen_port))
        .merge(("address", configuration.listen_address));
    rocket::custom(figment).mount(
        "/",
        routes![
            users::create::create,
            users::login::login,
            users::logout::logout,
            users::modify::modify,
            users::delete::delete,
            users::whoami::whoami,
            users::list::list,
            events::create::create,
            events::delete::delete,
            events::modify::modify,
            events::list::list,
            hello_world,
        ],
    )
}
