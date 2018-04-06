// Logging
#[macro_use]
extern crate log;

// Configuration reading support
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::error::Error;

mod config;

pub fn run(config_filename: String) -> Result<(), Box<Error>> {
    debug!("Got parameter \"{}\"", config_filename);
    let config = config::parse_config(config_filename);
    if let Err(e) = config {
        error!("Error while parsing config: {}", e);
        return Err(e);
    }
    let config = config.unwrap();
    debug!("Got token \"{}\" from config file", config.token.unwrap());
    return Ok(());
}