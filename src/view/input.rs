use model::Event;
use view::terminal::Terminal;

use std::sync::mpsc::Sender;

use termbuf::termion::event::Key;
use termbuf::TermSize;

use failure::Error;

const BOTTOM_START: usize = 3;
const SIDE_PADDING: usize = 3;

enum State {
    Message,
    Command,
}

pub struct Input {
    text: String,
    state: State,
    typing: bool,
    event_channel: Sender<Event>,
}

impl Input {
    pub fn new(event_channel: Sender<Event>) -> Input {
        Input {
            text: String::new(),
            state: State::Message,
            typing: true,
            event_channel,
        }
    }

    pub fn render(&self, screen: &mut Terminal, size: TermSize) {
        let saturated_side = (size.width as usize).saturating_sub(SIDE_PADDING * 2);
        let clipped_text = if self.text.len() > saturated_side {
            let clip = self
                .text
                .chars()
                .skip(
                    self.text
                        .len()
                        .saturating_sub(saturated_side.saturating_sub(1)),
                )
                .collect::<String>();

            "…".to_owned() + &clip
        } else {
            self.text.clone()
        };

        screen.buf.draw_box(
            SIDE_PADDING - 1,
            size.height.saturating_sub(BOTTOM_START + 1),
            saturated_side,
            1,
        );

        screen.buf.print(
            SIDE_PADDING,
            size.height.saturating_sub(BOTTOM_START),
            &clipped_text,
        );

        screen.buf.set_cursor_position(
            SIDE_PADDING + clipped_text.len() + 1,
            size.height.saturating_sub(BOTTOM_START) + 1,
        );
    }

    pub fn submit(&mut self) -> Result<(), Error> {
        let event = match self.state {
            State::Message => Event::UserMessage(self.text.clone()),
            State::Command => Event::UserCommand(self.text[1..].to_owned()),
        };
        self.state = State::Message;
        self.event_channel.send(event)?;
        self.text.clear();
        Ok(())
    }

    pub fn key_press(&mut self, key: Key) -> Result<(), Error> {
        match key {
            Key::Backspace | Key::Delete => {
                if self.text.len() == 1 {
                    self.state = State::Message
                }
                let _ = self.text.pop();
            }
            Key::Esc => self.state = State::Message,
            Key::Char('\n') => {
                self.submit()?;
            }
            Key::Char('\t') => {
                // Glitching out, to be used for channel switching
            }
            Key::Char(ch) => {
                if ch == ':' && self.text.is_empty() {
                    self.state = State::Command
                }
                if self.typing {
                    if self.text.is_empty() {
                        self.typing = false;
                    }
                } else {
                    self.event_channel.send(Event::UserTyping).unwrap();
                    self.typing = true;
                }
                self.text.push(ch);
            }
            Key::Ctrl('u') => self.text.clear(),
            _ => {}
        }
        Ok(())
    }
}
