use std::{process::exit, sync::Mutex};

use app::App;
use common::configuration::loader::{load, load_config};
use lazy_static::lazy_static;

pub mod app;
pub mod call;
pub mod widgets;

lazy_static! {
    static ref TOKEN: Mutex<String> = Mutex::new(String::new());
}
lazy_static! {
    static ref API_URL: Mutex<String> = Mutex::new(String::new());
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = match load() {
        Ok(conf) => load_config(conf),
        Err(e) => {
            eprintln!("Error while loading configuration: {e}");
            exit(1)
        }
    };
    *TOKEN.lock().unwrap() = config.token;
    *API_URL.lock().unwrap() = config.api_link;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
