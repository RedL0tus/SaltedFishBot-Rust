//! Bot API wrapper

// Dependencies
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;

// Dependencies of hyper
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

pub struct Bot {
    pub token: String,
    pub username: String,
}

impl Bot {
    pub fn new(token: &String, username: &String) -> Result<Bot, &'static str>{
        return Ok(
            Bot {
                token: token.clone(),
                username: username.clone(),
            }
        );
    }
}