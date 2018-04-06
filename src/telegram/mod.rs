//! Telegram bot

// Telegram bot api
extern crate futures;
extern crate tokio_core;
extern crate telebot;

use telebot::bot;
use tokio_core::reactor::Core;                       
use futures::stream::Stream;
use futures::Future;
use telebot::functions::*;

// Configuration
use super::config;

// Error handling
use std::error::Error;

/// Startup
pub fn startup(config: config::Config) -> Result<(), Box<Error>> {
    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), &config.token.unwrap())
        .update_interval(200);

    let handle = bot.new_cmd("/reply")
        .and_then(|(bot, msg)| {
            let mut text = msg.text.unwrap().clone();
            if text.is_empty() {
                text = "<empty>".into();
            }

            bot.message(msg.chat.id, text).send()
        });

    bot.register(handle);

    bot.run(&mut lp).unwrap();
    return Ok(());
}