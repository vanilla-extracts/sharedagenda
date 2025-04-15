use std::process::exit;

use configuration::{load, write_default_config};
use database::Database;
use users::{creation::create, delete::delete, login::login, logout::logout, modify::modify};

#[macro_use]
extern crate rocket;

pub mod configuration;
pub mod database;
mod events;
mod users;

#[launch]
fn rocket() -> _ {
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

    let database = Database::new();
    match database.setup_database() {
        Ok(_) => println!("Tables have been created."),
        Err(e) => {
            println!("{e}");
            exit(1)
        }
    };

    let figment = rocket::Config::figment()
        .merge(("port", configuration.listen_port))
        .merge(("address", configuration.listen_address));
    rocket::custom(figment).mount("/", routes![create, login, logout, modify, delete])
}
