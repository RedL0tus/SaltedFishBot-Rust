//! Configuration save & load utility

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
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Config {
    pub token: Option<String>,
    pub username: Option<String>,
}

impl Config {
    /// Generate new config
    pub fn new(token: &String, username: &String) -> Result<Config, &'static str> {
        if token.len() > 0 && username.len() > 0{
            return Ok(
                Config {
                    token: Some(token.to_string()),
                    username: Some(username.to_string()),
                }
            )
        } else {
            return Err("Invalid token or username");
        }
    }

    /// Write config to file at given location
    pub fn write(&self, location: &String) -> Result<(), Box<Error>> {
        let content = toml::to_string(&self);
        if let Err(e) = content {
            return Err(Box::new(e));
        }
        let content: String = content.unwrap();
        let buffer = File::create(location);
        if let Err(e) = buffer {
            return Err(Box::new(e));
        }
        let mut buffer = buffer.unwrap();
        if let Err(e) = buffer.write(content.as_bytes()) {
            return Err(Box::new(e));
        }
        return Ok(());
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
    debug!("Got: {}", content);
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
            let mut buffer = fs::File::create("test_parse.toml").unwrap();
            buffer.write(b"token = 'test'\nusername = 'testusername'").unwrap();
            assert_eq!(
                parse_config("test_parse.toml".to_string()).unwrap(),
                Config {
                    token: Some("test".to_string()),
                    username: Some("testusername".to_string()),
                }
            );
        }
        fs::remove_file("test_parse.toml").unwrap();
    }

    // Test config generating functionality
    #[test]
    fn config_generate() {
        assert_eq!(
            Config::new(&"test".to_string(), &"testusername".to_string()).unwrap(),
            Config {
                token: Some("test".to_string()),
                username: Some("testusername".to_string()),
            }
        );
    }

    // Test config writing functionality
    #[test]
    fn config_write() {
        let config = Config::new(&"test".to_string(), &"testusername".to_string()).unwrap();
        config.write(&"test_generate.toml".to_string()).unwrap();
        let mut file = fs::File::open("test_generate.toml").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(
            content,
            "token = \"test\"\nusername = \"testusername\"\n".to_string()
        );
        fs::remove_file("test_generate.toml").unwrap();
    }
}