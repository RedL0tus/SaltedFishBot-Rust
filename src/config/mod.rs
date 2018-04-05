extern crate toml;

use toml::Value;

// Bot configuration
#[derive(Debug)]
pub struct Config {
    token: String,
}

/// Reads config (TOML) from file
/// 
/// # Examples
/// 
/// ```
/// parse_config("config.toml");
/// ```
pub fn parse_config(config_filename: String) -> Config {
    debug!("Reading config from: {}", config_filename);
    let value = "foo = 'bar'".parse::<Value>().unwrap();
    return Config{ token: "test".to_string() };
}