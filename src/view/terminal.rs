use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use termbuf::termion::async_stdin;
use termbuf::termion::input::TermRead;
use termbuf::TermBuf;

use models::event::Event;

use failure::Error;

pub struct Terminal {
    pub buf: TermBuf,
}

impl Terminal {
    pub fn new() -> Result<Terminal, Error> {
        let mut term = Terminal {
            buf: TermBuf::init()?,
        };
        term.buf.set_cursor_visible(true);
        Ok(term)
    }

    pub fn listen(&self, event_channel: Sender<Event>, killswitch: Receiver<()>) {
        thread::spawn(move || loop {
            let mut keys = async_stdin().keys();
            loop {
                if let Some(Ok(key)) = keys.next() {
                    event_channel.send(Event::Keypress(key)).unwrap();
                } else if killswitch.try_recv().is_ok() {
                    trace!("Input loop received killsignal");
                    break;
                } else {
                    thread::sleep(Duration::from_millis(50));
                }
            }
        });
    }
}
