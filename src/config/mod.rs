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
    pub port: Option<i16>,
}

impl Config {
    /// Generate new config
    pub fn new(token: &String, port: &i16) -> Result<Config, &'static str> {
        if token.len() > 0 {
            Ok(
                Config {
                    token: Some(token.clone()),
                    port: Some(port.clone()),
                }
            )
        } else {
            Err("Invalid token")
        }
    }

    /// Write config to file at given location
    pub fn write(&self, location: &String) -> Result<(), Box<Error>> {
        let content = toml::to_string(&self)?;
        let mut buffer = File::create(location)?;
        buffer.write(content.as_bytes())?;
        Ok(())
    }
}

/// Reads config (TOML) from file
pub fn parse_config(config_filename: String) -> Result<Config, Box<Error>> {
    debug!("Reading config from: {}", config_filename);
    // Read from file
    let mut file = File::open(config_filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    debug!("Got: {}", content);
    // Parsing
    let config: Config = toml::from_str(&content)?;
    Ok(config)
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
            buffer.write(b"token = \"test\"\nport = 8090").unwrap();
            assert_eq!(
                parse_config("test.toml".to_string()).unwrap(),
                Config {
                    token: Some("test".to_string()),
                    port: Some(8090),
                }
            );
        }
        fs::remove_file("test.toml").unwrap();
    }

    // Test config generating functionality
    #[test]
    fn config_generate() {
        assert_eq!(
            Config::new(&"test".to_string(), &8090).unwrap(),
            Config {
                token: Some("test".to_string()),
                port: Some(8090)
            }
        );
    }

    // Test config writing functionality
    #[test]
    fn config_write() {
        let config = Config::new(&"test".to_string(), &8090).unwrap();
        config.write(&"test.toml".to_string()).unwrap();
        let mut file = fs::File::open("test.toml").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(
            content,
            "token = \"test\"\nport = 8090\n".to_string()
        );
    }
}
