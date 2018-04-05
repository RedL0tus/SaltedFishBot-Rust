#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate salted_fish_bot;

use std::env;
use std::process;
use std::path::Path;
use clap::{Arg, App, SubCommand};

fn main() {
    // Use clap for command line support
    let matches = App::new("SaltedFishBot-Rust")
        .version("0.1.0")
        .author("KayMW <redl0tus@noreply.github.com>")
        .about("Yet another salted fish bot written in Rust.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file.")
            .takes_value(true))
        .subcommand(SubCommand::with_name("generate_config")
            .about("Generates config file")
            .arg(Arg::with_name("token")
                .short("t")
                .help("Bot token from BotFather")
                .takes_value(true)))
        .get_matches();
    // Initialize logger
    if let Err(_) = env::var("SALTED_BOT_LOG"){
        env::set_var("SALTED_BOT_LOG", "info");
    }
    env_logger::init_from_env("SALTED_BOT_LOG");
    info!("Starting up...");
    // Find config file from command line option
    let config_filename: String;
    match matches.value_of("config") {
        Some(_) => config_filename = matches.value_of("config").unwrap().to_string(),
        None => config_filename = String::from("config.toml"),
    }
    // Check if the config file exists or not
    if ! Path::new(&config_filename).exists() {
        error!("Config file \"{}\" not found!", config_filename);
        process::exit(1);
    }
    // Run the main program
    info!("Using config from {}.", config_filename);
    if let Err(e) = salted_fish_bot::run(config_filename) {
        error!("Application error: {}", e);
        process::exit(2);
    }
}
