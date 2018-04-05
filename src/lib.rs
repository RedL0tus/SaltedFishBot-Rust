#[macro_use]
extern crate log;

// Configuration reading support
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::error::Error;

mod config;

pub fn run(config_filename: String) -> Result<(), Box<Error>>{
    debug!("Got {}", config_filename);
    let config = config::parse_config(config_filename);
    debug!("Got {:?}", config);
    return Ok(());
}