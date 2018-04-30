use serenity::model::channel;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::id::{ChannelId, MessageId, UserId};
use serenity::utils::Colour;
use termbuf::termion::{color,  style};
use termbuf::TermSize;
use textwrap::fill;

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::io;

use discord::utils;
use models::application::Application;
use models::message::MessageItem;
use view::terminal::Terminal;

const LEFT_PADDING: usize = 20;
const RIGHT_PADDING: usize = 5;
const TIME_PADDING: usize = 3;
const LEFT_START: usize = 0;
const TOP_START: usize = 5;
const BOTTOM_DIFF: usize = 10;

fn color_to_8bit(colour: ::serenity::utils::Colour) -> color::AnsiValue {
    color::AnsiValue::rgb(
        (u16::from(colour.r()) * 5 / 255) as u8,
        (u16::from(colour.g()) * 5 / 255) as u8,
        (u16::from(colour.b()) * 5 / 255) as u8,
    )
}

pub struct Messages {
    pub messages: RefCell<Vec<MessageItem>>,
    timestamp_fmt: String,
    truecolor: bool,
    nickname_cache: RefCell<HashMap<UserId, (String, Option<Colour>)>>,
}

impl Messages {
    pub fn new(timestamp_fmt: String) -> Messages {
        let truecolor = match env::var("COLORTERM") {
            Ok(term) => term.to_lowercase() == "truecolor",
            Err(_) => false,
        };

        Messages {
            messages: RefCell::new(Vec::new()),
            timestamp_fmt,
            truecolor,
            nickname_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_msg(&self, msg: MessageItem) {
        self.messages.borrow_mut().push(msg);
    }

    pub fn delete_msg(&self, channel_id: ChannelId, message_id: MessageId) {
        let mut msg_index = None;
        for (i, msg) in self.messages.borrow().iter().enumerate() {
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
            self.messages.borrow_mut().remove(index);
        }
    }

    pub fn delete_msg_bulk(&self, channel_id: ChannelId, message_ids: &[MessageId]) {
        debug!(
            "Bulk delete: {}",
            message_ids
                .iter()
                .map(|msg_id| msg_id.0.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        self.messages.borrow_mut().retain(|msg| match msg {
            MessageItem::DiscordMessage(msg) => {
                msg.channel_id != channel_id && !message_ids.contains(&msg.id)
            }
        });
    }

    pub fn update_message(&self, update: MessageUpdateEvent) {
        for mut msg in self.messages.borrow_mut().iter_mut() {
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

    pub fn load_messages(&self, app: &Application) {
        use serenity::builder::GetMessages;

        let num = app.view.terminal_size.height;
        let retriever = GetMessages::default().limit(num as u64);
        if let Some(channel) = app.current_channel {
            self.messages.borrow_mut().clear();

            for message in channel
                .messages(|_| retriever)
                .unwrap()
                .iter()
                .rev()
                .cloned()
            {
                self.add_msg(MessageItem::DiscordMessage(Box::new(message)));
            }
        }
    }

    // pub fn load_messages(&mut self, channel: Option<ChannelId>, num: usize) {
    //     use serenity::builder::GetMessages;

    //     let retriever = GetMessages::default().limit(num as u64);
    //     if let Some(channel) = channel {
    //         self.messages.get_mut().clear();

    //         for message in channel
    //             .messages(|_| retriever)
    //             .unwrap()
    //             .iter()
    //             .rev()
    //             .cloned()
    //         {
    //             self.add_msg(MessageItem::DiscordMessage(Box::new(message)));
    //         }
    //     }
    // }

    fn colorize_nick(&self, message: &channel::Message) -> String {
        let mut cache = self.nickname_cache.borrow_mut();
        let entry = cache.entry(message.author.id);

        use std::collections::hash_map::Entry::*;
        let (nick, colour) = match entry {
            Occupied(o) => o.into_mut(),
            Vacant(v) => {
                if let Some(member) = utils::member(&message) {
                    v.insert((
                        member
                            .nick
                            .clone()
                            .unwrap_or_else(|| message.author.name.to_owned()),
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

    pub fn render(&self, screen: &mut Terminal, size: TermSize) -> Result<(), io::Error> {
        let rough_msg_count = size.height;
        let mut msgs = self.messages.borrow_mut();
        let msg_diff = msgs.len().saturating_sub(rough_msg_count as usize);

        msgs.drain(0..msg_diff);

        let mut messages = msgs.clone();

        let mut y = size.height as usize - BOTTOM_DIFF - 1;
        for mut msg in messages.iter_mut().rev() {
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
        &self,
        msg: &mut channel::Message,
        size: TermSize,
        screen: &mut Terminal,
        y: &mut usize,
    ) -> Result<bool, io::Error> {
        // Show an indicator if an attachement is present
        let content = if !msg.attachments.is_empty() {
            "📎 ".to_owned() + &msg.content_safe()
        } else {
            msg.content_safe()
        };

        let wrapped_lines: Vec<String> = content
            .lines()
            .map(|line| {
                fill(
                    line,
                    (size.width as usize)
                        .saturating_sub(RIGHT_PADDING + LEFT_PADDING + LEFT_START + TIME_PADDING),
                )
            })
            .collect();
        msg.content = wrapped_lines.join("\n");

        let lines: Vec<_> = msg.content.lines().rev().collect();
        for (i, line) in lines.iter().enumerate() {
            if i == (lines.len() - 1) {
                screen.buf.put_string(
                    // &format!("{}:", self.colorize_nick(&msg)),
                    &format!("{}:", msg.author.name),
                    (LEFT_START) as u16,
                    (*y + TOP_START) as u16,
                );

                screen.buf.put_string(
                    &format!(
                        "{}{}",
                        msg.timestamp
                            .with_timezone(&::chrono::offset::Local)
                            .format(&self.timestamp_fmt),
                        if msg.edited_timestamp.is_some() {
                            "*"
                        } else {
                            ""
                        }
                    ),
                    size.width.saturating_sub(RIGHT_PADDING as u16) as u16,
                    (*y + TOP_START) as u16,
                )
            }
            screen.buf.put_string(
                line,
                (LEFT_PADDING + LEFT_START) as u16,
                (*y + TOP_START) as u16,
            );
            if *y == 0 {
                return Ok(false);
            }
            *y -= 1;
        }
        Ok(true)
    }
}
