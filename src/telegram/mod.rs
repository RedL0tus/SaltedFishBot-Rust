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

// Strings
use super::strings;

// Error handling
use std::error::Error;

// RefCell
use std::cell::RefCell;

use std::fmt;

// Avoid E0117 for User
struct DisplayWrapperUser (telegram_bot::types::User);

impl fmt::Display for DisplayWrapperUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref username) = self.0.username{
            write!(f, "@{} ({})", username, self.0.id)
        } else {
            write!(f, "ID {}", self.0.id)
        }
    }
}

// Avoid E0117 for MessageChat
struct DisplayWrapperChat (telegram_bot::types::MessageChat);

impl fmt::Display for DisplayWrapperChat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            telegram_bot::types::MessageChat::Private(_) => write!(f, "Private chat"),
            telegram_bot::types::MessageChat::Group(ref group) => write!(f, "{} ({})", group.title, group.id),
            telegram_bot::types::MessageChat::Supergroup(ref group) => {
                if let Some(ref username) = group.username {
                    write!(f , "{} (@{}, {})", group.title, username, group.id)
                } else {
                    write!(f , "{} ({})", group.title, group.id)
                }
            },
            _ => write!(f, "Unknown"),
        }
    }
}


/// Startup
pub fn startup(config: config::Config, strings: strings::Strings) -> Result<(), Box<Error>> {
    let mut core = Core::new().unwrap();
    // Initialize telegram_bot instance
    let api = Api::configure(config.token.unwrap()).build(core.handle()).unwrap();
    // Fetch bot's username before start
    let bot_name: RefCell<Option<String>> = {
        if let Some(name) = core.run(api.send(GetMe)).unwrap().username {
            RefCell::new(Some(format!("@{}",name).into()))
        } else {
            RefCell::new(None)
        }
    };
    info!("Username set to {}", &bot_name.borrow().as_ref().unwrap());
    // Setup the main loop: Fetch new updates via long poll method
    let main_loop = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            //let message = RefCell::new(message);
            if let MessageKind::Text {ref data, ref entities, ..} = message.kind {
                info!("[{}] {}: {}", DisplayWrapperChat(message.chat.clone()), DisplayWrapperUser(message.from.clone()), &data);
                // Use entities to determine the message contains a command or not
                for entity in entities.iter() {
                    debug!("Received entities: {:?}", entity);
                    if entity.offset == 0 && entity.kind == telegram_bot::types::MessageEntityKind::BotCommand {
                        // Route message to command router
                        command_router(bot_name.borrow().clone(), message.clone(), data, &api, strings.clone()).map_err(|e|
                            // Send error message before quitting
                            api.spawn(message.text_reply(format!("{}: {}", strings.clone().error, e).to_string()))
                        ).expect("Fail to process commands");
                    }
                }
            }
        }
        Ok(())
    });
    info!("Waiting for requests...");
    // Start the actuall loop using tokio-core
    core.run(main_loop)?;
    Ok(())
}

/// Command router
fn command_router(bot_name: Option<String>, message: telegram_bot::Message, data: &String, api: &telegram_bot::Api, strings: strings::Strings) -> Result<(), Box<Error>>{
    // Get the first part of the command
    let mut content = data.split_whitespace();
    if let Some(mut cmd) = content.next() {
        // Remove bot's username before processing commands
        if let Some(name) = bot_name {
            if cmd.ends_with(name.as_str()) {
                cmd = cmd.rsplitn(2, '@').skip(1).next().unwrap();
            }
        }
        // The actuall router
        match cmd {
            "/echo" => command_echo(message.clone(), &api, strings.clone())?,
            _ => (),
        } 
    }
    Ok(())
}

/// Implementation of /echo
fn command_echo(message: telegram_bot::Message, api: &telegram_bot::Api, strings: strings::Strings) -> Result<(), Box<Error>> {
    // Cut the first part before processing the message
    if let MessageKind::Text {ref data, ..} = message.kind {
        let mut content = data.split_whitespace();
        let mut response = String::new();
        if content.clone().count() == 1 {
            // If nothing exists
            response = strings.clone().echo_empty.into();
        } else {
            content.next();
            for i in content.clone() {
                response.push_str(&format!("{} ", i).as_str());
            }
            let len = response.len();
            response.truncate(len - 1);
        }
        // Send the response
        info!("Response length: {}", response.len());
        api.spawn(message.text_reply(
            format!("{} 說：“{}”", DisplayWrapperUser(message.from.clone()), response)
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use telegram_bot::*;
    use super::*;

    #[test]
    fn username() {
        let user: types::User = types::User {
            id: types::UserId::new(114514810),
            first_name: String::from("田所"),
            last_name: Some(String::from("浩二")),
            username: Some(String::from("YJSNPI"))
        };
        assert_eq!(
            "@YJSNPI (114514810)",
            format!("{}", DisplayWrapperUser(user))
        );
    }

    #[test]
    fn user_id_only() {
        let user: types::User = types::User {
            id: types::UserId::new(114514810),
            first_name: String::from("田所"),
            last_name: None,
            username: None
        };
        assert_eq!(
            "ID 114514810",
            format!("{}", DisplayWrapperUser(user))
        );
    }
}