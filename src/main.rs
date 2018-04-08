#![feature(iterator_flatten)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate rustbox;
extern crate serenity;
extern crate toml;

use failure::Error;

use rustbox::RustBox;
use serenity::prelude::*;

use std::env;
use std::sync::{mpsc, Arc, Mutex};

mod communication;
mod errors;
mod event;
mod ui;

use errors::*;

lazy_static! {
    static ref MESSAGE_CHANNEL: Arc<Mutex<mpsc::Sender<communication::ChannelMessage>>> =
        Arc::new(Mutex::new(mpsc::channel().0));
}

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

    let (tx, rx) = mpsc::channel();
    *MESSAGE_CHANNEL.lock().unwrap() = tx;

    let mut client = match Client::new(&config.token, event::Handler) {
        Ok(client) => client,
        Err(err) => return Err(InternalSerenityError::from(err))?,
    };

    let shard_manager = client.shard_manager.clone();

    let message_area = ui::layout::Rect::new(0, 0, rustbox.width(), rustbox.height());
    let mut messages = ui::messages::Messages::new();

    let rustbox = Arc::new(rustbox);
    let rustbox_event_loop = rustbox.clone();
    std::thread::spawn(move || loop {
        match rustbox_event_loop.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                rustbox::Key::Char('q') => {
                    MESSAGE_CHANNEL
                        .lock()
                        .unwrap()
                        .send(communication::ChannelMessage::ShutdownAll)
                        .unwrap();
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    });

    std::thread::spawn(move || {
        // Start new shard
        match client.start() {
            Ok(_) => {}
            Err(_) => {}
            // Err(err) => return Err(InternalSerenityError::from(err)).unwrap(),
        };
    });

    loop {
        use communication::ChannelMessage::*;
        match rx.recv() {
            Ok(ShutdownAll) => {
                shard_manager.lock().shutdown_all();
                break;
            }
            Ok(NewMessage(msg)) => {
                messages.add_msg(msg);
            }
            _ => {}
        }
        rustbox.clear();
        messages.render(&message_area, &rustbox);
        rustbox.present();
    }
    Ok(())
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
