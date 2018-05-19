//! Telegram bot

// Telegram bot api
extern crate futures;
extern crate tokio_core;
extern crate telebot;

use telebot::bot;
use tokio_core::reactor::Core;                       
use futures::stream::Stream;
use telebot::functions::*;

// Configuration
use super::config;

// Error handling
use std::error::Error;

// Error messages
const ERROR: &'static str = "發生了什麼不得了的事情，請聯繫 @TheSaltedFish";

/// Startup
pub fn startup(config: config::Config) -> Result<(), Box<Error>> {
    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), &config.token.unwrap())
        .update_interval(200);
    let handle = bot.new_cmd("/echo")
        .and_then(|(bot, msg)| {
            if let Some(res) = command_echo(&msg) {
                bot.message(msg.chat.id, res).send()
            } else {
                error!("");
                bot.message(msg.chat.id, ERROR.to_string()).send()
            }
        });
    bot.register(handle);

    bot.run(&mut lp).unwrap();
    return Ok(());
}

/// Get username
pub fn get_username(user: telebot::objects::User) -> String {
    if let Some(username) = user.username {
        String::from(format!("@{} ({})", username, user.id))
    } else {
        user.id.to_string()
    }
}

pub fn command_echo(msg: &telebot::objects::Message) -> Option<String> {
    let mut text = msg.text.clone().unwrap().clone();
    let username = get_username(msg.from.clone().unwrap().clone());
    info!("Received /echo from {}: {}", &username, &text);
    if text.is_empty() {
        text = "<什麼也沒有>".into();
    }
    Some(format!("{} 說： {}", &username, &text).into())
}