#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate salted_fish_bot;

use std::env;
use std::process;
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
        .subcommand(SubCommand::with_name("run")
            .about("Run the bot"))
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
        Some(filename) => config_filename = filename.to_string(),
        None => config_filename = String::from("config.toml"),
    }
    // When the subcommand is `run`
    if let Some(_) = matches.subcommand_matches("run") {
        // Run the main program
        info!("Reading config from {}", config_filename);
        if let Err(e) = salted_fish_bot::run(config_filename) {
            error!("Application error: {}", e);
            process::exit(1);
        }
        process::exit(0);
    }
    // When the subcommand is `generate_config`
    if let Some(matches) = matches.subcommand_matches("generate_config") {
        // Generate config
        info!("Generating config from command line arguments...");
        if let None =  matches.value_of("token") {
            error!("Token not found in arguments.");
            process::exit(2);
        }
        // Bring the configuration utility into scope
        use salted_fish_bot::config::*;
        // Using the configuration utility to save config
        let config = Config::new(&matches.value_of("token").unwrap().to_string()).expect("Invalid token");
        config.write(&config_filename).expect("Cannot write config");
        info!("Done, configuration saved to {}", config_filename);
        process::exit(0);
    }
    info!("No command specified, use --help for more information");
    process::exit(0);
}
