#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;

use std::error::Error;

pub fn run(config_filename: String) -> Result<(), Box<Error>>{
    info!("Got {}", config_filename);
    return Ok(());
}