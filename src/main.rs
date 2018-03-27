#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serenity;
extern crate toml;

use failure::Error;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use std::env;

mod errors;

use errors::*;

#[derive(Debug, Clone, Deserialize)]
struct Config {
    token: String,
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        println!(">> {}", msg.content);
    }

    // Called when discord responds READY
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);
        main_loop();
    }
}

fn main_loop() {}

fn load_config() -> Result<Config, Error> {
    use std::fs::File;
    use std::io::Read;

    let home_dir = env::home_dir().ok_or(HomeDirError)?;
    let mut file = File::open(home_dir.join(".config/ded/config.toml"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    Ok(config)
}

fn run() -> Result<(), Error> {
    let config = load_config()?;

    let mut client = match Client::new(&config.token, Handler) {
        Ok(client) => client,
        Err(err) => return Err(InternalSerenityError::from(err))?,
    };

    // Start new shard
    println!("Starting...");

    match client.start() {
        Ok(_) => Ok(()),
        Err(err) => return Err(InternalSerenityError::from(err))?,
    }
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("An error occurred");
            eprintln!("{}", e.cause());
            std::process::exit(1);
        }
    }
}
