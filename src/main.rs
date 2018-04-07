#![feature(iterator_flatten)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate rustbox;
extern crate serenity;
extern crate toml;

use failure::Error;

use rustbox::RustBox;
use serenity::prelude::*;

use std::env;

mod errors;
mod event;
mod ui;

use errors::*;

#[derive(Debug, Clone, Deserialize)]
struct Config {
    token: String,
}

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

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let message_area = ui::layout::Rect::new(5, 0, rustbox.width() - 10, rustbox.height() - 5);
    let mut messages = ui::messages::Messages::new();

    messages.add_msg("Hello, World!".to_owned());
    messages.render(&message_area, &rustbox);

    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                rustbox::Key::Char('q') => return Ok(()),
                _ => {}
            },
            _ => {}
        }
    }

    let mut client = match Client::new(&config.token, event::Handler) {
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
