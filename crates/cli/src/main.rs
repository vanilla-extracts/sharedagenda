use std::{
    env::{self, Args},
    io::{self, BufRead},
    process::exit,
};

use ansi_term::Color;
use atty::Stream;
use configuration::loader::{Configuration, load, load_config, write_default_config};
use linefeed::{Interface, ReadResult};

static VERSION: &str = "v0.1.0";

mod configuration;

fn main() {
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
                println!("");
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
                println!("");
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
            "exit" => break,
            "help" => {
                println!("-----SharedAgenda CLI Help-----");
                println!("");
                println!("help          > show the help");
                println!("config        > prints the configuration file path");
                println!("version       > prints the version");
                println!("api [url]     > set the URL for which API to use");
                println!("register      > register a new account for sharedagenda");
                println!("login         > login with your account");
                println!("new|create    > create a new event");
                println!("list <date>   > prints out the list of events");
                println!("change        > change user information");
                println!("modify        > modify event information");
                println!("pretty <date> > pretty print the list of events");
                println!("");
                println!("-----SharedAgenda CLI Help-----");
            }
            _ => println!("SOON"),
        }
    }
}
