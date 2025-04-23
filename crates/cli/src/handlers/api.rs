use crate::{
    API_URL,
    configuration::loader::{load, write_config},
};

pub fn api(url: &str) {
    let mut config = load().unwrap_or_default();
    config.api_link = url.to_string();
    match write_config(&config) {
        Ok(_) => {
            println!("Configuration has been updated with new URL: {}", url);
            API_URL.with(|f| {
                *f.borrow_mut() = url.to_string();
            });
        }
        Err(_) => {
            println!("Error while updating configuration")
        }
    }
}
