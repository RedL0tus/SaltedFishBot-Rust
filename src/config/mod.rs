// Parse config using crate toml
extern crate toml;

// Error handling
use std::error::Error;

// Reading file from filesystem
use std::fs::File;
use std::io::prelude::*;

// Bot configuration
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Deserialize)]
pub struct Config {
    pub token: Option<String>,
}

impl Config {
    pub fn new(token: &String) -> Result<Config, &'static str> {
        if token.len() > 0 {
            return Ok(
                Config {
                    token: Some(token.clone())
                }
            )
        } else {
            return Err("Invalid token");
        }
    }
}

/// Reads config (TOML) from file
pub fn parse_config(config_filename: String) -> Result<Config, Box<Error>> {
    debug!("Reading config from: {}", config_filename);
    // Read from file
    let file = File::open(config_filename);
    if let Err(e) = file {
        error!("Error while reading file: {}", e);
        return Err(Box::new(e));
    };
    let mut content = String::new();
    if let Err(e) = file.unwrap().read_to_string(&mut content) {
        error!("Error while reading file: {}", e);
        return Err(Box::new(e));
    };
    // Parsing
    let config: Config = toml::from_str(&content).unwrap();
    return Ok(config);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    // Test config parsing functionality
    #[test]
    fn config_parser() {
        {
            let mut buffer = fs::File::create("test.toml").unwrap();
            buffer.write(b"token = 'test'").unwrap();
            assert_eq!(
                parse_config("test.toml".to_string()).unwrap(),
                Config {
                    token: Some("test".to_string()),
                }
            );
        }
        fs::remove_file("test.toml").unwrap();
    }
}