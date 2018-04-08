use rustbox::{self, Color, RustBox};
use serenity::model::channel;
use std::borrow::Cow;

pub struct Messages {
    pub messages: Vec<channel::Message>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages {
            messages: Vec::new(),
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

    pub fn render(&mut self, area: &super::layout::Rect, rustbox: &RustBox) {
        let rough_msg_count = area.height;
        let msg_diff = self.messages.len().saturating_sub(rough_msg_count);

        self.messages.drain(0..msg_diff);

        let mut unfolded_msgs = self.messages.clone();
        for mut msg in &mut unfolded_msgs {
            let wrapped_lines: Vec<String> = msg.content
                .lines()
                .map(|line| Self::wrap(line, area.width).join("\n"))
                .collect();
            msg.content = wrapped_lines.join("\n");
        }

        let mut y = area.height - 1;
        for message in unfolded_msgs.iter().rev() {
            for line in message.content.lines().rev() {
                rustbox.print(
                    area.x,
                    y + area.y,
                    rustbox::RB_NORMAL,
                    Color::Default,
                    Color::Default,
                    line,
                );
                if y == 0 {
                    return;
                }
                y -= 1;
            }
        }
    }
}
