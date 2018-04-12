use std::io::{self, stdin, stdout, Error, Stdin, Stdout, Write};

use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread;

use termion::async_stdin;
use termion::input::Keys;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use models::event::Event;

pub struct Terminal {
    pub screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Terminal, Error> {
        Ok(Terminal {
            screen: AlternateScreen::from(stdout().into_raw_mode()?),
        })
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
                }
            }
        });
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.screen.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.screen.flush()
    }
}
