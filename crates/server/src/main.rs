use std::process::exit;

use configuration::{load, write_default_config};
use database::Database;
use users::{login::login, logout::logout, modify::modify};

#[macro_use]
extern crate rocket;

pub mod configuration;
pub mod database;
mod events;
pub mod structs;
mod users;

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
            login,
            logout,
            modify,
            users::delete::delete,
            events::create::create,
            events::delete::delete,
        ],
    )
}
