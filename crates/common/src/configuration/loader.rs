use ansi_term::{ANSIGenericString, Color};
use serde::{Deserialize, Serialize};

use crate::VERSION;

#[derive(Serialize, Deserialize, Clone)]
pub struct Greeting {
    pub greeting_message: String,
    pub greeting_colour: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub prompt_message: String,
    pub prompt_colour: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub api_link: String,
    pub token: String,
    pub greeting: Greeting,
    pub prompt: Prompt,
}

#[derive(Clone, Debug)]
pub struct Loaded<'a> {
    pub api_link: String,
    pub token: String,
    pub greeting_message: ANSIGenericString<'a, str>,
    pub prompt_message: String,
    pub prompt_colour: Color,
}

impl Default for Greeting {
    fn default() -> Self {
        Self {
            greeting_message:
                "Welcome to SharedAgenda CLI REPL Version %version%, type help for help".to_string(),
            greeting_colour: "blue".to_string(),
        }
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            prompt_message: "> ".to_string(),
            prompt_colour: "red".to_string(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            api_link: "http://localhost:8000".to_string(),
            prompt: Prompt::default(),
            token: String::new(),
            greeting: Greeting::default(),
        }
    }
}

pub fn load() -> Result<Configuration, confy::ConfyError> {
    let cfg = confy::load("sharedagenda", Some("cli"));
    match cfg {
        Ok(config) => Ok(config),
        _ => Ok(Configuration::default()),
    }
}

pub fn write_config(c: &Configuration) -> Result<(), confy::ConfyError> {
    confy::store("sharedagenda", Some("cli"), c)?;
    Ok(())
}

pub fn write_default_config() -> Result<(), confy::ConfyError> {
    write_config(&Configuration::default())
}

pub fn load_rgb_color(str: &str) -> (u8, u8, u8) {
    let first = &str[0..2];
    let second = &str[2..4];
    let last = &str[4..6];

    let rd = u8::from_str_radix(first, 16);
    let gd = u8::from_str_radix(second, 16);
    let bd = u8::from_str_radix(last, 16);

    let r = rd.unwrap_or(0xFF);
    let g = gd.unwrap_or(0xFF);
    let b = bd.unwrap_or(0xFF);

    (r, g, b)
}

pub fn load_color(string: String) -> Color {
    match string.to_lowercase().as_str() {
        "purple" => Color::Purple,
        "cyan" => Color::Cyan,
        "blue" => Color::Blue,
        "black" => Color::Black,
        "red" => Color::Red,
        "yellow" => Color::Yellow,
        "green" => Color::Green,
        "white" => Color::White,
        s => {
            if s.starts_with("#") {
                let str = s.strip_prefix("#");
                if str.unwrap().len() < 6 {
                    Color::Cyan
                } else {
                    let (r, g, b) = load_rgb_color(str.unwrap());
                    if r == 0xFF && g == 0xFF && b == 0xFF {
                        Color::Cyan
                    } else {
                        Color::RGB(r, g, b)
                    }
                }
            } else {
                Color::Cyan
            }
        }
    }
}

pub fn replace_variable(str: String) -> String {
    str.replace("%author%", "Charlotte Thomas")
        .replace("%version%", VERSION)
        .to_string()
}

pub fn load_config<'a>(config: Configuration) -> Loaded<'a> {
    Loaded {
        token: config.token,
        greeting_message: load_color(config.greeting.greeting_colour)
            .paint(replace_variable(config.greeting.greeting_message)),
        prompt_message: config.prompt.prompt_message,
        prompt_colour: load_color(config.prompt.prompt_colour),
        api_link: config.api_link,
    }
}
