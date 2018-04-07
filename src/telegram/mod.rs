//! Telegram bot

// Telegram bot api
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;

// Configuration
use super::config;

// Error handling
use std::error::Error;

// Bot API
mod api;

/// Startup
pub fn startup(config: config::Config) -> Result<(), Box<Error>> {
    debug!("Creating bot with given config");
    let bot = api::Bot::new(&config.token.unwrap(), &config.username.unwrap());
    return Ok(());
}