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
    token: Option<String>,
}

/// Reads config (TOML) from file
pub fn parse_config(config_filename: String) -> Result<Config, Box<Error>> {
    debug!("Reading config from: {}", config_filename);
    // Read from file
    let file = File::open(config_filename);
    if let Err(error) = file {
        error!("Error while reading file: {}", error);
        return Err(Box::new(error));
    };
    let mut content = String::new();
    if let Err(error) = file.unwrap().read_to_string(&mut content) {
        error!("Error while reading file: {}", error);
        return Err(Box::new(error));
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