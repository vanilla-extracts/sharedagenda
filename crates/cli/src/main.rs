use std::{process::exit, sync::Mutex};

use crate::handlers::api::api;
use configuration::loader::{load, load_config, write_default_config};
use handlers::{
    change::change, create::create, delete::delete, event_deletion::remove, list::list,
    login::login, logout::logout, register::register,
};
use lazy_static::lazy_static;
use linefeed::{Interface, ReadResult};

static VERSION: &str = "v1.0.0-beta";
lazy_static! {
    static ref TOKEN: Mutex<String> = Mutex::new(String::new());
}
lazy_static! {
    static ref API_URL: Mutex<String> = Mutex::new(String::new());
}

mod configuration;
mod handlers;

#[tokio::main]
async fn main() {
    let config = match load() {
        Ok(config) => config,
        Err(_) => {
            let _ = write_default_config();
            match load() {
                Ok(cfg) => cfg,
                Err(_) => {
                    println!("fatal error please remove your config file");
                    exit(1);
                }
            }
        }
    };

    let loaded = load_config(config);

    println!("{}", loaded.greeting_message);

    *API_URL.lock().unwrap() = loaded.api_link;
    *TOKEN.lock().unwrap() = loaded.token;

    let interface = Interface::new("sharedagenda").unwrap();
    let style = &loaded.prompt_colour;
    let prompt = &loaded.prompt_message;
    interface
        .set_prompt(&format!(
            "\x01{prefix}\x02{prompt}\x01{suffix}\x02",
            prefix = style.prefix(),
            prompt = prompt,
            suffix = style.suffix()
        ))
        .unwrap();

    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        match line.as_str().trim() {
            "version" => println!("SharedAgenda CLI REPL {VERSION}"),
            "config" => println!("$HOME/.config/sharedagenda/cli.toml"),
            "token" => println!("Current Token is: {}", TOKEN.lock().unwrap()),
            "exit" => break,
            "help" => {
                println!("-----SharedAgenda CLI REPL Help-----");
                println!("> help                                                  > show the help");
                println!(
                    "> config                                                > prints the configuration file path"
                );
                println!(
                    "> version                                               > prints the version"
                );
                println!(
                    "> api [url]                                             > set the URL for which API to use"
                );
                println!(
                    "> register <name>%<email>%<password>                    > register a new account for sharedagenda"
                );
                println!(
                    "> login <email>%<password>                              > login with your account"
                );
                println!(
                    "> logout                                                > logout of your account"
                );
                println!(
                    "> delete                                                > delete your account"
                );
                println!(
                    "> remove <id>                                           > remove an event"
                );
                println!(
                    "> new|create <name>%<date_start>%<date_end>%[invitees]  > create a new event"
                );
                println!(
                    "> list <date>                                           > prints out the list of events"
                );
                println!(
                    "> change <name>%<email>%<password>                      > change user information"
                );
                println!(
                    "> modify <name>%<date_start>%<date_end>                 > modify event information"
                );
                println!(
                    "> pretty <date>                                         > pretty print the list of events"
                );
                println!("-----SharedAgenda CLI REPL Help-----");
            }
            str if str.starts_with("api") => match str.strip_prefix("api") {
                Some(str) if str.trim() != "" => {
                    api(str.trim());
                }
                _ => {
                    println!("Current API URL: {}", API_URL.lock().unwrap())
                }
            },
            str if str.starts_with("register") => match str.strip_prefix("register") {
                Some(reg) if reg.trim() != "" => register(reg.trim()).await,
                _ => println!("Usage: register <name>%<email>%<password>"),
            },
            str if str.starts_with("change") => match str.strip_prefix("change") {
                Some(reg) if reg.trim() != "" => change(reg.trim()).await,
                _ => println!("Usage: change <name>%<email>%<password>"),
            },
            str if str.starts_with("login") => match str.strip_prefix("login") {
                Some(log) if log.trim() != "" => login(log.trim()).await,
                _ => println!("Usage: login <email>%<password>"),
            },
            str if str.starts_with("logout") => logout().await,
            str if str.starts_with("delete") => delete().await,
            str if str.starts_with("list") => match str.strip_prefix("list") {
                Some(time) => list(time.trim().to_string()).await,
                _ => list("".to_string()).await,
            },
            str if str.starts_with("create") => match str.strip_prefix("create") {
                Some(s) if s.trim() != "" => create(s).await,
                _ => println!("Usage: create <name>%<date_start>%<date_end>%[invitees]"),
            },
            str if str.starts_with("new") => match str.strip_prefix("new") {
                Some(s) if s.trim() != "" => create(s).await,
                _ => println!("Usage: new <name>%<date_start>%<date_end>%[invitees]"),
            },
            str if str.starts_with("remove") => match str.strip_prefix("remove") {
                Some(s) if s.trim() != "" => remove(s).await,
                _ => println!("Usage: remove <id>"),
            },
            _ => println!("SOON"),
        }
        interface.add_history_unique(line);
    }
}
