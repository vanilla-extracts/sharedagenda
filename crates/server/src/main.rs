use std::{process::exit, thread::sleep, time::Duration};

use configuration::{load, write_default_config};
use database::Database;
use rocket::tokio;
use systemd::daemon::{STATE_WATCHDOG, notify};

#[macro_use]
extern crate rocket;

pub mod configuration;
pub mod database;
mod events;
pub mod structs;
mod users;

#[launch]
async fn rocket() -> _ {
    async fn keep_alive() {
        loop {
            if let Err(e) = notify(false, [(STATE_WATCHDOG, "1")].iter()) {
                println!("Error while sending the keepalive: {e}");
                exit(1);
            }
            sleep(Duration::from_secs(5));
        }
    }

    tokio::spawn(keep_alive());

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
            events::create::create,
            events::delete::delete,
            events::modify::modify,
            events::list::list,
        ],
    )
}
