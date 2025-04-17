use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub listen_address: String,
    pub listen_port: i64,
}

impl Default for DatabaseConfiguration {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: "5432".to_string(),
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "agenda".to_string(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            database: DatabaseConfiguration::default(),
            listen_address: "0.0.0.0".to_string(),
            listen_port: 8000,
        }
    }
}

pub fn write_config(c: &Configuration) -> Result<(), confy::ConfyError> {
    confy::store("sharedagenda", Some("config"), c)?;
    Ok(())
}

pub fn write_default_config() -> Result<(), confy::ConfyError> {
    write_config(&Configuration::default())
}

pub fn load() -> Result<Configuration, confy::ConfyError> {
    let res = confy::load("sharedagenda", Some("config"))?;
    Ok(res)
}
