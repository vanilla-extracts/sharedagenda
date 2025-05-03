use std::{env, io::BufRead, process::exit, str::Chars, sync::Mutex};

use crate::handlers::api::api;
use atty::Stream;
use configuration::loader::{load, load_config, write_default_config};
use handlers::{
    create::create, delete::delete, event_deletion::remove, list::list, login::login,
    logout::logout, modify::modify, register::register, whoami::whoami,
};
use lazy_static::lazy_static;
use linefeed::{Interface, ReadResult};

static VERSION: &str = "v2.1.0-dev";
lazy_static! {
    static ref TOKEN: Mutex<String> = Mutex::new(String::new());
}
lazy_static! {
    static ref API_URL: Mutex<String> = Mutex::new(String::new());
}

mod configuration;
mod handlers;

pub fn parse_line_into_arguments(line: &str) -> Vec<String> {
    let mut args = vec![];
    //"Mon Nom" email@test.fr tortue => ["Mon Nom", "email@test.fr", "tortue"]

    fn parse_string(mut acc: Vec<String>, chars: &mut Chars) -> String {
        for char in chars.by_ref() {
            match char {
                '"' => {
                    break;
                }
                s => acc.push(s.to_string()),
            }
        }
        acc.join("")
    }

    let mut chars = line.chars();
    let mut acc: Vec<String> = vec![];
    while let Some(char) = chars.next() {
        match char {
            '"' => {
                let word: String = parse_string(vec![], &mut chars);
                args.push(word);
            }
            ' ' => {
                let word = acc.join("");
                if word.trim() != "" {
                    args.push(word);
                    acc = vec![]
                }
            }
            e => {
                acc.push(e.to_string());
            }
        }
    }
    if !acc.is_empty() {
        args.push(acc.join(""));
    }
    args
}

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

    *API_URL.lock().unwrap() = loaded.api_link;
    *TOKEN.lock().unwrap() = loaded.token;

    let mut args = env::args();
    if args.len() > 1 || !atty::is(Stream::Stdin) {
        let mut a = vec![];
        if !atty::is(Stream::Stdin) {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                a.push(line.unwrap());
            }
        } else {
            args.nth(0);
            args.for_each(|f| a.push(f));
        }
        let first = a.remove(0);

        match first.as_str() {
            "-v" | "--version" => println!("SharedAgenda CLI {VERSION}"),
            "config" => println!("$HOME/.config/sharedagenda/cli.toml"),
            "token" => println!("Current Token is: {}", TOKEN.lock().unwrap()),
            "api" => api(&a.join("")),
            "register" => register(a).await,
            "login" => login(a).await,
            "logout" => logout().await,
            "delete" => delete().await,
            "remove" => remove(&a.join("")).await,
            "new" | "create" => create(a).await,
            "list" => list(a.join("")).await,
            "whoami" => whoami().await,
            "modify" => modify(a).await,
            _ => {
                println!("-----SharedAgenda CLI Help-----");
                println!(
                    "sharedagenda help                                                  > shows the help"
                );
                println!(
                    "sharedagenda config                                                > prints the configuration file path"
                );
                println!(
                    "sharedagenda version                                               > prints the version"
                );
                println!(
                    "sharedagenda api [url]                                             > sets the URL for which API to use"
                );
                println!(
                    "sharedagenda register <name> <email> <password>                    > registers a new account for sharedagenda"
                );
                println!(
                    "sharedagenda login <email> <password>                              > login with your account"
                );
                println!(
                    "sharedagenda logout                                                > logout of your account"
                );
                println!(
                    "sharedagenda delete                                                > deletes your account"
                );
                println!(
                    "sharedagenda remove <id>                                           > removes an event"
                );
                println!(
                    "sharedagenda new|create <name> <date_start> <date_end> [invitees]  > creates a new event"
                );
                println!(
                    "sharedagenda list <date>                                           > prints out the list of events"
                );
                println!(
                    "sharedagenda whoami                                                > prints user informations"
                );

                println!(
                    "sharedagenda modify <name> <email> <password>                      > modifies user information"
                );
                println!("-----SharedAgenda CLI REPL Help-----");
            }
        }
        exit(0);
    }

    println!("{}", loaded.greeting_message);
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
                println!(
                    "> help                                                  > shows the help"
                );
                println!(
                    "> config                                                > prints the configuration file path"
                );
                println!(
                    "> version                                               > prints the version"
                );
                println!(
                    "> api [url]                                             > sets the URL for which API to use"
                );
                println!(
                    "> register <name> <email> <password>                    > registers a new account for sharedagenda"
                );
                println!(
                    "> login <email> <password>                              > login with your account"
                );
                println!(
                    "> logout                                                > logout of your account"
                );
                println!(
                    "> delete                                                > deletes your account"
                );
                println!(
                    "> remove <id>                                           > removes an event"
                );
                println!(
                    "> new|create <name> <date_start> <date_end> [invitees]  > creates a new event"
                );
                println!(
                    "> list <date>                                           > prints out the list of events"
                );
                println!(
                    "> whoami                                                > prints user informations"
                );

                println!(
                    "> modify <name> <email> <password>                      > modifies user information"
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
                Some(reg) if reg.trim() != "" => {
                    register(parse_line_into_arguments(reg.trim())).await
                }
                _ => println!("Usage: register <name> <email> <password>"),
            },
            str if str.starts_with("modify") => match str.strip_prefix("modify") {
                Some(reg) if reg.trim() != "" => {
                    modify(parse_line_into_arguments(reg.trim())).await
                }
                _ => println!("Usage: modify <name> <email> <password>"),
            },
            str if str.starts_with("login") => match str.strip_prefix("login") {
                Some(log) if log.trim() != "" => login(parse_line_into_arguments(log.trim())).await,
                _ => println!("Usage: login <email> <password>"),
            },
            str if str.starts_with("logout") => logout().await,
            str if str.starts_with("whoami") => whoami().await,
            str if str.starts_with("delete") => delete().await,
            str if str.starts_with("list") => match str.strip_prefix("list") {
                Some(time) => list(time.trim().to_string()).await,
                _ => list("".to_string()).await,
            },
            str if str.starts_with("create") => match str.strip_prefix("create") {
                Some(s) if s.trim() != "" => create(parse_line_into_arguments(s.trim())).await,
                _ => println!("Usage: create <name> <date_start> <date_end> [invitees]"),
            },
            str if str.starts_with("new") => match str.strip_prefix("new") {
                Some(s) if s.trim() != "" => create(parse_line_into_arguments(s.trim())).await,
                _ => println!("Usage: new <name> <date_start> <date_end> [invitees]"),
            },
            str if str.starts_with("remove") => match str.strip_prefix("remove") {
                Some(s) if s.trim() != "" => remove(s.trim()).await,
                _ => println!("Usage: remove <id>"),
            },
            _ => println!("SOON"),
        }
        interface.add_history_unique(line);
    }
}

#[cfg(test)]
mod test {
    use crate::parse_line_into_arguments;

    #[test]
    fn test_parse_line() {
        let expected = vec![
            "Mon Nom".to_string(),
            "email@tortue.fr".to_string(),
            "tortue".to_string(),
        ];
        let value = parse_line_into_arguments("\"Mon Nom\" email@tortue.fr tortue");
        println!("{:#?}", value);
        assert_eq!(expected, value);
    }
}
