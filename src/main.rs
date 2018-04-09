#![feature(iterator_flatten)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate serenity;
extern crate termion;
extern crate toml;

use failure::Error;

use serenity::prelude::*;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

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
    #[serde(default = "timestamp_default")]
    timestamp_fmt: String,
}

fn timestamp_default() -> String {
    "%-I:%-M".to_string()
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

    if !termion::is_tty(&stdout()) {
        println!("This requires an interactive tty");
        return Ok(());
    }

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let term_size = termion::terminal_size().unwrap();

    let (tx, rx) = mpsc::channel();
    *MESSAGE_CHANNEL.lock().unwrap() = tx;

    let mut client = match Client::new(&config.token, event::Handler) {
        Ok(client) => client,
        Err(err) => return Err(InternalSerenityError::from(err))?,
    };

    let shard_manager = client.shard_manager.clone();

    let message_area = ui::layout::Rect::new(0, 5, term_size.0 as usize, term_size.1 as usize - 10);
    let mut messages = ui::messages::Messages::new(config.timestamp_fmt);

    std::thread::spawn(move || loop {
        use termion::event::Key::*;
        for c in stdin().keys() {
            match c {
                Ok(Char('q')) => {
                    MESSAGE_CHANNEL
                        .lock()
                        .unwrap()
                        .send(communication::ChannelMessage::ShutdownAll)
                        .unwrap();
                    break;
                }
                _ => {}
            }
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
        write!(screen, "{}", termion::clear::All);
        messages.render(&message_area, &mut screen);
        screen.flush().unwrap();
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
