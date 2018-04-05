#[macro_use]
extern crate log;
extern crate toml;

use std::error::Error;

mod config;

pub fn run(config_filename: String) -> Result<(), Box<Error>>{
    debug!("Got {}", config_filename);
    let config = config::parse_config(config_filename);
    debug!("Got {:?}", config);
    return Ok(());
}