//! Load strings from file

extern crate toml;

// Error handling
use std::error::Error;

// Reading file from filesystem
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Strings {
    pub name: String,
    // Error message
    pub error: String,
    // /echo
    pub echo: String,
    pub echo_empty: String,
}

impl Strings {
    /// Read strings from files
    pub fn load() -> Result<Strings, Box<Error>> {
        debug!("Reading language strings from strings.toml");
        // Read from file
        let mut file = File::open("strings.toml")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        debug!("Got: {}", content);
        // Parsing
        let strings: Strings = toml::from_str(&content)?;
        Ok(strings)
    }
}