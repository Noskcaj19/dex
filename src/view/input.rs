use models::event::Event;
use view::terminal::{Terminal, TerminalSize};

use std::io::Write;
use std::sync::mpsc::Sender;

use termion::cursor;
use termion::event::Key;

use failure::Error;

const BOTTOM_START: usize = 3;
const SIDE_PADDING: usize = 3;

pub struct Input {
    text: String,
    event_channel: Sender<Event>,
}

impl Input {
    pub fn new(event_channel: Sender<Event>) -> Input {
        Input {
            text: String::new(),
            event_channel,
        }
    }

    pub fn render(&self, screen: &mut Terminal, size: TerminalSize) {
        trace!("Current buffer: {}", self.text);
        let clipped_text = if self.text.len() > (size.width - SIDE_PADDING * 2) {
            let clip = self.text
                .chars()
                .skip(
                    self.text
                        .len()
                        .saturating_sub(size.width - (SIDE_PADDING * 2) - 1),
                )
                .collect::<String>();

            "…".to_owned() + &clip
        } else {
            self.text.clone()
        };

        write!(
            screen,
            "{}{}",
            cursor::Goto(1 + SIDE_PADDING as u16, (size.height - BOTTOM_START) as u16),
            clipped_text,
        ).unwrap();
    }

    pub fn submit(&mut self) -> Result<(), Error> {
        self.text.clear();
        self.event_channel
            .send(Event::UserCommand(self.text.clone()))?;
        Ok(())
    }

    pub fn key_press(&mut self, key: Key) -> Result<(), Error> {
        match key {
            Key::Backspace | Key::Delete => {
                let _ = self.text.pop();
            }
            Key::Char('\n') => {
                self.submit()?;
            }
            Key::Char(ch) => {
                self.text.push(ch);
            }
            Key::Ctrl('u') => self.text.clear(),
            _ => {}
        }
        Ok(())
    }
}
