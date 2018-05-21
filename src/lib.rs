// Logging
#[macro_use]
extern crate log;

// Configuration reading support
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;

// Telegram bot
extern crate futures;
extern crate tokio_core;
extern crate telegram_bot;

// Multithreading
use std::thread;

use std::error::Error;

pub mod config;
pub mod telegram;

pub fn run(config_filename: String) -> Result<(), Box<Error>> {
    debug!("Got parameter \"{}\"", config_filename);
    let config = config::parse_config(config_filename)?;
    debug!("Got token \"{:?}\" from config file", &config.token);
    debug!("Starting telegram bot...");
    let handle_telegram = thread::spawn(move || {
        telegram::startup(config.clone()).unwrap();
    });
    handle_telegram.join().unwrap();
    Ok(())
}