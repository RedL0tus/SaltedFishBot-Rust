//! Telegram bot

// Telegram bot api
extern crate futures;
extern crate tokio_core;
extern crate telegram_bot;

// Dependencies of telegram_bot
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

// Configuration
use super::config;

// Error handling
use std::error::Error;

// RefCell
use std::cell::RefCell;

// Error messages
const ERROR: &'static str = "發生了什麼不得了的事情，請聯繫 @TheSaltedFish";

/// Startup
pub fn startup(config: config::Config) -> Result<(), Box<Error>> {
    let mut core = Core::new().unwrap();
    let api = Api::configure(config.token.unwrap()).build(core.handle()).unwrap();
    let bot_name: RefCell<Option<String>> = {
        if let Some(name) = core.run(api.send(GetMe)).unwrap().username {
            RefCell::new(Some(format!("@{}",name).into()))
        } else {
            RefCell::new(None)
        }
    };
    info!("Username set to {}", &bot_name.borrow().as_ref().unwrap());
    // Fetch new updates via long poll method
    let main_loop = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            //let message = RefCell::new(message);
            if let MessageKind::Text {ref data, ref entities, ..} = message.kind {
                let requestee = get_username(message.from.clone());
                let from: String = {
                    match message.chat {
                        MessageChat::Private(_) => "Private chat".into(),
                        MessageChat::Group(ref group) => format!("{} ({})", group.title, group.id).into(),
                        MessageChat::Supergroup(ref group) => format!("{} ({})", group.title, group.id).into(),
                        _ => "Unknown".into(),
                    }
                };
                info!("[{}] {}: {}", from, requestee, data);
                for entity in entities.iter() {
                    debug!("Received entities: {:?}", entity);
                    if entity.offset == 0 && entity.kind == telegram_bot::types::MessageEntityKind::BotCommand {
                        process_commands(bot_name.borrow().clone(), message.clone(), data, &api).map_err(|e|
                            api.spawn(message.text_reply(format!("{}: {}", &ERROR, e).to_string()))
                        ).expect("Fail to process commands");
                    }
                }
            }
        }
        Ok(())
    });
    info!("Waiting for requests...");
    core.run(main_loop)?;
    Ok(())
}

/// Command router
fn process_commands(bot_name: Option<String>, message: telegram_bot::Message, data: &String, api: &telegram_bot::Api) -> Result<(), Box<Error>>{
    let mut content = data.split_whitespace();
    if let Some(mut cmd) = content.next() {
        if let Some(name) = bot_name {
            if cmd.ends_with(name.as_str()) {
                cmd = cmd.rsplitn(2, '@').skip(1).next().unwrap();
            }
        }
        match cmd {
            "/echo" => command_echo(message.clone(), &api)?,
            _ => (),
        } 
    }
    Ok(())
}

/// Get username
fn get_username(user: telegram_bot::types::User) -> String {
    if let Some(username) = user.username {
        String::from(format!("@{} ({})", username, user.id))
    } else {
        user.id.to_string()
    }
}

fn command_echo(message: telegram_bot::Message, api: &telegram_bot::Api) -> Result<(), Box<Error>> {
    if let MessageKind::Text {ref data, ..} = message.kind {
        let mut content = data.split_whitespace();
        let mut response = String::new();
        if content.clone().count() == 1 {
            response = "<什麼也沒有>".into();
        } else {
            content.next();
            for i in content.clone() {
                response.push_str(&format!("{} ", i).as_str());
            }
        }
        api.spawn(message.text_reply(
            response
        ));
    }
    Ok(())
}