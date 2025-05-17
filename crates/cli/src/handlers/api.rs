use common::configuration::loader::{load, write_config};

use crate::API_URL;

pub fn api(url: &str) {
    let mut config = load().unwrap_or_default();
    config.api_link = url.to_string();
    match write_config(&config) {
        Ok(_) => {
            println!("Configuration has been updated with new URL: {}", url);
            *API_URL.lock().unwrap() = url.to_string();
        }
        Err(_) => {
            println!("Error while updating configuration")
        }
    }
}
