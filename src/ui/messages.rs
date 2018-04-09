use serenity::model::channel;
use std::borrow::Cow;
use std::io::{Stdout, Write};
use termion::{color, cursor, style};

const LEFT_PADDING: usize = 12;
const RIGHT_PADDING: usize = 12;

pub struct Messages {
    pub messages: Vec<channel::Message>,
    timestamp_fmt: String,
}

fn color_to_8bit(colour: ::serenity::utils::Colour) -> color::AnsiValue {
    color::AnsiValue::rgb(
        (colour.r() as u16 * 5 / 255) as u8,
        (colour.g() as u16 * 5 / 255) as u8,
        (colour.b() as u16 * 5 / 255) as u8,
    )
}

fn colorize_name(message: &channel::Message) -> String {
    match ::utils::member(&message).and_then(|member| member.colour()) {
        Some(colour) => {
            if *::SUPPORTS_TRUECOLOR {
                format!(
                    "{}{}{}",
                    color::Fg(color::Rgb(colour.r(), colour.g(), colour.b())),
                    message.author.name,
                    style::Reset,
                )
            } else {
                format!(
                    "{}{}{}",
                    color::Fg(color_to_8bit(colour)),
                    message.author.name,
                    style::Reset,
                )
            }
        }
        None => message.author.name.to_string(),
    }
}

impl Messages {
    pub fn new(timestamp_fmt: String) -> Messages {
        Messages {
            messages: Vec::new(),
            timestamp_fmt,
        }
    }

    pub fn add_msg(&mut self, msg: channel::Message) {
        self.messages.push(msg);
    }

    fn wrap<'a>(string: &'a str, length: usize) -> Vec<Cow<'a, str>> {
        string
            .as_bytes()
            .chunks(length)
            .map(String::from_utf8_lossy)
            .collect()
    }

    pub fn render(&mut self, area: &super::layout::Rect, screen: &mut Stdout) {
        let rough_msg_count = area.height;
        let msg_diff = self.messages.len().saturating_sub(rough_msg_count);

        self.messages.drain(0..msg_diff);

        let mut unfolded_msgs = self.messages.clone();
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
                        format!("{}:", colorize_name(&message))
                    );

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
                    );
                }
                write!(
                    screen,
                    "{}{}",
                    cursor::Goto((LEFT_PADDING + area.x) as u16, (y + area.y) as u16),
                    line
                );
                if y == 0 {
                    return;
                }
                y -= 1;
            }
        }
    }
}
