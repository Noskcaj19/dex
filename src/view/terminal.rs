use std::io::{self, stdout, Error, Stdout, Write};

use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use serenity::prelude::Mutex;

use termion;
use termion::async_stdin;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use models::event::Event;

pub struct Terminal {
    pub screen: Mutex<AlternateScreen<RawTerminal<Stdout>>>,
}

#[derive(Clone, Copy, Debug)]
pub struct TerminalSize {
    pub width: usize,
    pub height: usize,
}

impl Terminal {
    pub fn new() -> Result<Terminal, Error> {
        Ok(Terminal {
            screen: Mutex::new(AlternateScreen::from(stdout().into_raw_mode()?)),
        })
    }

    pub fn size(&self) -> TerminalSize {
        let size = termion::terminal_size().unwrap();
        TerminalSize {
            width: size.0 as usize,
            height: size.1 as usize,
        }
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
                thread::sleep(Duration::from_millis(50));
            }
        });
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.screen.lock().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.screen.lock().flush()
    }
}
