use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;

lazy_static! {
    pub static ref CONFIG: Settings = Settings::init_config();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoConfig {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfig {
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub debug: bool,
    pub database: MongoConfig,
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub session: SessionConfig,
}

impl Settings {
    fn init_config() -> Self {
        println!("CONFIG INIT");
        let mut s = Config::default();
        let mut config_file_path = env::current_dir().expect("Cannot get current path");

        // Get current RUN_MODE, should be: development/production
        let current_env = env::var("RUN_MODE").unwrap_or(String::from("development"));

        config_file_path.push("players");
        config_file_path.push("environments");
        config_file_path.push(format!("{}.yaml", current_env));

        // Add in the current environment file
        // Default to 'development' env
        s.merge(File::from(config_file_path).required(true))
            .expect("Could not read file");

        // Add in settings from the environment
        // DEBUG=1 sets debug key, DATABASE_URL sets database.url key
        s.merge(Environment::new().prefix("APP").separator("_"))
            .expect("Cannot get env");

        // Deserialize configuration
        let r: Settings = s.try_into().expect("Configuration error");

        r
    }
}