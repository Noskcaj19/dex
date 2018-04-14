use serenity::model::channel;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::id::{ChannelId, MessageId, UserId};
use serenity::utils::Colour;
use termion::{color, cursor, style};
use textwrap::fill;

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

use discord::utils;
use models::message::MessageItem;
use view::terminal::{Terminal, TerminalSize};

const LEFT_PADDING: usize = 20;
const RIGHT_PADDING: usize = 5;
const TIME_PADDING: usize = 3;
const LEFT_START: usize = 0;
const TOP_START: usize = 5;
const BOTTOM_DIFF: usize = 10;

fn color_to_8bit(colour: ::serenity::utils::Colour) -> color::AnsiValue {
    color::AnsiValue::rgb(
        (colour.r() as u16 * 5 / 255) as u8,
        (colour.g() as u16 * 5 / 255) as u8,
        (colour.b() as u16 * 5 / 255) as u8,
    )
}

pub struct Messages {
    pub messages: Vec<MessageItem>,
    timestamp_fmt: String,
    truecolor: bool,
    nickname_cache: HashMap<UserId, (String, Option<Colour>)>,
}

impl Messages {
    pub fn new(timestamp_fmt: String) -> Messages {
        let truecolor = match env::var("COLORTERM") {
            Ok(term) => term.to_lowercase() == "truecolor",
            Err(_) => false,
        };

        Messages {
            messages: Vec::new(),
            timestamp_fmt,
            truecolor,
            nickname_cache: HashMap::new(),
        }
    }

    pub fn add_msg(&mut self, msg: MessageItem) {
        self.messages.push(msg);
    }

    pub fn delete_msg(&mut self, channel_id: ChannelId, message_id: MessageId) {
        let mut msg_index = None;
        for (i, msg) in self.messages.iter().enumerate() {
            match msg {
                MessageItem::DiscordMessage(msg) => {
                    debug!("Deleting message: {}", message_id);
                    if msg.channel_id == channel_id && msg.id == message_id {
                        msg_index = Some(i);
                        break;
                    }
                }
            }
        }
        if let Some(index) = msg_index {
            self.messages.remove(index);
        }
    }

    pub fn delete_msg_bulk(&mut self, channel_id: ChannelId, message_ids: Vec<MessageId>) {
        debug!(
            "Bulk delete: {}",
            message_ids
                .iter()
                .map(|msg_id| msg_id.0.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        self.messages.retain(|msg| match msg {
            MessageItem::DiscordMessage(msg) => {
                !(msg.channel_id == channel_id) && !message_ids.contains(&msg.id)
            }
        });
    }

    pub fn update_message(&mut self, update: MessageUpdateEvent) {
        for mut msg in self.messages.iter_mut() {
            match msg {
                MessageItem::DiscordMessage(ref mut msg) => {
                    if update.id == msg.id && update.channel_id == msg.channel_id {
                        debug!("Updated message: {}", msg.id);
                        utils::update_msg(msg, update);
                        break;
                    }
                }
            }
        }
    }

    fn colorize_nick(&mut self, message: &channel::Message) -> String {
        let entry = self.nickname_cache.entry(message.author.id);
        use std::collections::hash_map::Entry::*;
        let (nick, colour) = match entry {
            Occupied(o) => o.into_mut(),
            Vacant(v) => {
                if let Some(member) = utils::member(&message) {
                    v.insert((
                        member
                            .nick
                            .clone()
                            .unwrap_or(message.author.name.to_owned()),
                        member.colour(),
                    ))
                } else {
                    v.insert((message.author.name.to_owned(), None))
                }
            }
        };

        match colour {
            Some(colour) => {
                if self.truecolor {
                    format!(
                        "{}{}{}",
                        color::Fg(color::Rgb(colour.r(), colour.g(), colour.b())),
                        nick,
                        style::Reset,
                    )
                } else {
                    format!(
                        "{}{}{}",
                        color::Fg(color_to_8bit(*colour)),
                        nick,
                        style::Reset,
                    )
                }
            }
            None => message.author.name.to_string(),
        }
    }

    pub fn render(&mut self, screen: &mut Terminal, size: TerminalSize) -> Result<(), io::Error> {
        let rough_msg_count = size.height;
        let msg_diff = self.messages.len().saturating_sub(rough_msg_count);

        self.messages.drain(0..msg_diff);

        let mut messages = self.messages.clone();

        let mut y = size.height - BOTTOM_DIFF - 1;
        for mut msg in &mut messages.iter_mut().rev() {
            match msg {
                MessageItem::DiscordMessage(msg) => {
                    if !self.render_discord_msg(msg, size, screen, &mut y)? {
                        break;
                    };
                }
            }
        }
        Ok(())
    }

    fn render_discord_msg(
        &mut self,
        msg: &mut channel::Message,
        size: TerminalSize,
        screen: &mut Terminal,
        y: &mut usize,
    ) -> Result<bool, io::Error> {
        let wrapped_lines: Vec<String> = msg.content
            .lines()
            .map(|line| {
                fill(
                    line,
                    size.width
                        .saturating_sub(RIGHT_PADDING + LEFT_PADDING + LEFT_START + TIME_PADDING),
                )
            })
            .collect();
        msg.content = wrapped_lines.join("\n");

        let lines: Vec<_> = msg.content.lines().rev().collect();
        for (i, line) in lines.iter().enumerate() {
            if i == (lines.len() - 1) {
                write!(
                    screen,
                    "{}{}",
                    cursor::Goto((LEFT_START) as u16, (*y + TOP_START) as u16),
                    format!("{}:", self.colorize_nick(&msg))
                )?;

                write!(
                    screen,
                    "{}{}",
                    cursor::Goto((size.width - RIGHT_PADDING) as u16, (*y + TOP_START) as u16),
                    msg.timestamp
                        .with_timezone(&::chrono::offset::Local)
                        .format(&self.timestamp_fmt)
                )?;
            }
            write!(
                screen,
                "{}{}",
                cursor::Goto((LEFT_PADDING + LEFT_START) as u16, (*y + TOP_START) as u16),
                line
            )?;
            if *y == 0 {
                return Ok(false);
            }
            *y -= 1;
        }
        Ok(true)
    }
}
