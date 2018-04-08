//! Bot API wrapper

// Dependencies
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper_rustls;

// Dependencies of hyper & hyper_rustls
use std::io;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json::Value;

pub struct Bot {
    pub token: String,
    pub username: String,
}

impl Bot {
    pub fn new(token: &String, username: &String) -> Bot {
        return Bot {
            token: token.clone(),
            username: username.clone(),
        };
    }

    pub fn get_updates(&self, offset: i64) {//-> Result<serde_json::Value, &'static str> {
        debug!("Offset: {}", offset);
        let mut core = Core::new().unwrap();
        debug!("tokio_core instance created");
        let client = Client::configure()
            .connector(hyper_rustls::HttpsConnector::new(4, &core.handle()))
            .build(&core.handle());
        debug!("hyper client configured");
        let uri = String::from(format!("https://api.telegram.org/bot{}/getUpdates?offset={}", &self.token, &offset)).parse().unwrap();
        debug!("URI set to {}", &uri);
        let work = client.get(uri).and_then(|res| {
            debug!("Response: {}", res.status());

            res.body().concat2().and_then(move |body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                })?;
                println!("{:?}", v);
                Ok(())
            })
        });
        core.run(work).unwrap();
    }
}