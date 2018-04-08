// Logging
#[macro_use]
extern crate log;

// Configuration reading support
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;

// Telegram bot
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper_rustls;

use std::error::Error;

pub mod config;
pub mod telegram;

pub fn run(config_filename: String) -> Result<(), Box<Error>> {
    debug!("Got parameter \"{}\"", config_filename);
    let config = config::parse_config(config_filename);
    if let Err(e) = config {
        error!("Error while parsing config: {}", e);
        return Err(e);
    }
    let config = config.unwrap();
    debug!("Got token \"{:?}\" from config file", &config.token);
    debug!("Got username \"{:?}\" from config file", &config.username);
    debug!("Starting telegram bot...");
    telegram::startup(config).unwrap();
    return Ok(());
}