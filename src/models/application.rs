use failure::Error;
use serenity;
use serenity::model::id::{ChannelId, GuildId};

use std::sync::mpsc::{self, Receiver, Sender};

use super::event::Event;
use super::preferences::Preferences;
use view::View;

pub struct Application {
    pub view: View,
    pub preferences: Preferences,
    pub current_guild: Option<GuildId>,
    pub current_channel: Option<ChannelId>,
    pub event_channel: Sender<Event>,
    events: Receiver<Event>,
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let preferences = Preferences::load()?;
        let (event_channel, events) = mpsc::channel();

        let view = View::new(event_channel.clone());

        Ok(Application {
            view,
            preferences,
            current_guild: None,
            current_channel: None,
            event_channel,
            events,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            debug!("Loop");
            // TODO: Render
            if !self.wait_for_event() {
                debug!("Exiting event loop");
                break;
            }
        }
        Ok(())
    }

    pub fn wait_for_event(&mut self) -> bool {
        use termion::event::Key;
        let event = self.events.recv();
        trace!("Event: {:?}", event);
        match event {
            Ok(Event::Keypress(key)) => match key {
                Key::Char('q') => return false,
                _ => {}
            },
            _ => {}
        }
        return true;
    }
}

/*
fn run() -> Result<(), Error> {
    let config = load_config()?;

    if !termion::is_tty(&stdout()) {
        eprintln!("This requires an interactive tty");
        return Err(NonInteractiveTty);
    }

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let terminal_size = termion::terminal_size().unwrap();

    // *CONTEXT.write() = Context {
    //     guild: config.guild,
    //     channel: config.channel,
    //     terminal_size,
    // };

    let (tx, rx) = mpsc::channel();
    // *MESSAGE_CHANNEL.lock() = tx;

    let handler = event::Handler(Arc::new(Mutex::new(tx)));
    let mut client = match Client::new(&config.token, handler) {
        Ok(client) => client,
        Err(err) => return Err(InternalSerenityError::from(err))?,
    };

    let message_area = ui::layout::Rect::new(
        0,
        5,
        terminal_size.0 as usize,
        terminal_size.1 as usize - 10,
    );
    let mut messages = ui::messages::Messages::new(config.timestamp_fmt);

    std::thread::spawn(move || loop {
        use termion::event::Key::*;
        for c in stdin().keys() {
            match c {
                Ok(Char('q')) => {
                    // MESSAGE_CHANNEL
                    //     .lock()
                    //     .send(communication::ChannelMessage::ShutdownAll)
                    //     .unwrap();
                    break;
                }
                _ => {}
            }
        }
    });

    let shard_manager = client.shard_manager.clone();
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
        write!(screen, "{}", termion::clear::All).unwrap();
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
*/
