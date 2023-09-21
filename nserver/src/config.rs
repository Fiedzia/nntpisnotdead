use serde::Deserialize;
use toml;

#[derive(Clone, Deserialize)]
pub struct Config {
   pub db: Db,
   pub server: Server,
}

#[derive(Clone, Deserialize)]
pub struct Db {
   pub username: String,
   pub password: String,
   pub database: String,
   pub host: String,
   pub port: u16,
   pub pool_size: u32,
}

#[derive(Clone, Deserialize)]
pub struct Server {
   pub host: String,
   pub port: u16,
}

pub fn load_config_from_file(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read config file {}: {:?}", path, e))?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| format!("Error parsing config.toml: {:?}", e))?;
    Ok(config)
}
