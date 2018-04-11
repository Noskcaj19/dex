use serenity::model::channel;
use serenity::model::id::{ChannelId, UserId};
use serenity::utils::Colour;
use termion::{color, cursor, style};

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Stdout, Write};

const LEFT_PADDING: usize = 20;
const RIGHT_PADDING: usize = 6;

fn color_to_8bit(colour: ::serenity::utils::Colour) -> color::AnsiValue {
    color::AnsiValue::rgb(
        (colour.r() as u16 * 5 / 255) as u8,
        (colour.g() as u16 * 5 / 255) as u8,
        (colour.b() as u16 * 5 / 255) as u8,
    )
}

pub struct Messages {
    pub messages: HashMap<ChannelId, Vec<channel::Message>>,
    timestamp_fmt: String,
    nickname_cache: HashMap<UserId, (String, Option<Colour>)>,
}

impl Messages {
    pub fn new(timestamp_fmt: String) -> Messages {
        Messages {
            messages: HashMap::new(),
            timestamp_fmt,
            nickname_cache: HashMap::new(),
        }
    }

    pub fn add_msg(&mut self, msg: channel::Message) {
        let messages = self.messages.entry(msg.channel_id).or_default();
        messages.push(msg);
    }

    fn wrap<'a>(string: &'a str, length: usize) -> Vec<Cow<'a, str>> {
        string
            .as_bytes()
            .chunks(length)
            .map(String::from_utf8_lossy)
            .collect()
    }

    fn colorize_nick(&mut self, message: &channel::Message) -> String {
        let entry = self.nickname_cache.entry(message.author.id);
        use std::collections::hash_map::Entry::*;
        let (nick, colour) = match entry {
            Occupied(o) => o.into_mut(),
            Vacant(v) => {
                if let Some(member) = ::utils::member(&message) {
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
                // if *::SUPPORTS_TRUECOLOR {
                //     format!(
                //         "{}{}{}",
                //         color::Fg(color::Rgb(colour.r(), colour.g(), colour.b())),
                //         nick,
                //         style::Reset,
                //     )
                // } else {
                format!(
                    "{}{}{}",
                    color::Fg(color_to_8bit(*colour)),
                    nick,
                    style::Reset,
                )
                // }
            }
            None => message.author.name.to_string(),
        }
    }

    pub fn render(&mut self, area: &::models::layout::Rect, screen: &mut Stdout) {
        return; /*
        let context = ::CONTEXT.read();
        if context.guild.is_none() || context.channel.is_none() {
            return;
        }

        let mut unfolded_msgs;
        {
            let messages = match self.messages.get_mut(&context.channel.unwrap()) {
                Some(messages) => messages,
                None => return,
            };

            let rough_msg_count = area.height;
            let msg_diff = messages.len().saturating_sub(rough_msg_count);

            messages.drain(0..msg_diff);

            unfolded_msgs = messages.clone();
        }

        for mut msg in &mut unfolded_msgs {
            let wrapped_lines: Vec<String> = msg.content
                .lines()
                .map(|line| {
                    Self::wrap(
                        line,
                        area.width.saturating_sub(RIGHT_PADDING + LEFT_PADDING),
                    ).join("\n")
                })
                .collect();
            msg.content = wrapped_lines.join("\n");
        }

        let mut y = area.height - 1;
        for message in unfolded_msgs.iter().rev() {
            let lines: Vec<_> = message.content.lines().rev().collect();
            for (i, line) in lines.iter().enumerate() {
                if i == (lines.len() - 1) {
                    write!(
                        screen,
                        "{}{}",
                        cursor::Goto(area.x as u16, (y + area.y) as u16),
                        format!("{}:", self.colorize_nick(&message))
                    ).unwrap();

                    write!(
                        screen,
                        "{}{}",
                        cursor::Goto(
                            (area.x + area.width - RIGHT_PADDING) as u16,
                            (y + area.y) as u16
                        ),
                        message
                            .timestamp
                            .with_timezone(&::chrono::offset::Local)
                            .format(&self.timestamp_fmt)
                    ).unwrap();
                }
                write!(
                    screen,
                    "{}{}",
                    cursor::Goto((LEFT_PADDING + area.x) as u16, (y + area.y) as u16),
                    line
                ).unwrap();
                if y == 0 {
                    return;
                }
                y -= 1;
            }
        }*/
    }
}
