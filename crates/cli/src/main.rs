use std::{
    env::{self, Args},
    io::{self, BufRead},
    process::exit,
    sync::Mutex,
};

use crate::handlers::api::api;
use atty::Stream;
use configuration::loader::{load, load_config, write_default_config};
use handlers::{delete::delete, login::login, logout::logout, register::register};
use lazy_static::lazy_static;
use linefeed::{Interface, ReadResult};

static VERSION: &str = "v0.1.0";
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
    let mut args: Args = env::args();
    if args.len() > 1 || !atty::is(Stream::Stdin) {
        let mut a: Vec<String> = vec![];

        if !atty::is(Stream::Stdin) {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                a.push(line.unwrap());
            }
        } else {
            let _ = args.next();
            args.for_each(|f| a.push(f));
        }

        match a.first().unwrap().as_str() {
            "-h" | "--help" => {
                println!("-----SharedAgenda CLI Help-----");
                println!("sharedagenda              > launch the REPL");
                println!("sharedagenda --help|-h    > show the help");
                println!("sharedagenda --conf|-c    > prints the configuration file path");
                println!("sharedagenda --version|-v > prints the version");
                println!("sharedagenda api [url]    > set the URL for which API to use");
                println!("sharedagenda register     > register a new account for sharedagenda");
                println!("sharedagenda login        > login with your account");
                println!("sharedagenda new|create   > create a new event");
                println!("sharedagenda list <date>  > prints out the list of events");
                println!("sharedagenda change       > change user information");
                println!("sharedagenda modify       > modify event information");
                println!("sharedagenda pretty       > pretty print the list of events");
                println!("-----SharedAgenda CLI Help-----");
            }
            "-v" | "--version" => {
                println!("SharedAgenda CLI {VERSION}");
            }
            "-c" | "--conf" => {
                println!("$HOME/.config/sharedagenda/cli.toml");
            }
            _ => {
                println!("SOON");
            }
        }
    }

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
                    "> register <name> <email> <password>                    > register a new account for sharedagenda"
                );
                println!(
                    "> login <email> <password>                              > login with your account"
                );
                println!(
                    "> logout                                                > logout of your account"
                );
                println!(
                    "> delete                                                > delete your account"
                );
                println!(
                    "> new|create <name> <date_start> <date_end> [invitees]  > create a new event"
                );
                println!(
                    "> list <date>                                           > prints out the list of events"
                );
                println!(
                    "> change <name> <email> <password>                      > change user information"
                );
                println!(
                    "> modify <name> <date_start> <date_end>                 > modify event information"
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
                _ => println!("Usage: register <name> <email> <password>"),
            },
            str if str.starts_with("login") => match str.strip_prefix("login") {
                Some(log) if log.trim() != "" => login(log.trim()).await,
                _ => println!("Usage: login <email> <password>"),
            },
            str if str.starts_with("logout") => logout().await,
            str if str.starts_with("delete") => delete().await,
            _ => println!("SOON"),
        }
        interface.add_history_unique(line);
    }
}
