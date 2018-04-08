use rustbox::{self, Color, RustBox};
use std::borrow::Cow;

pub struct Messages {
    pub messages: Vec<String>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages {
            messages: Vec::new(),
        }
    }

    pub fn add_msg(&mut self, msg: String) {
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

        let unfolded_msgs: Vec<_> = self.messages
            .iter()
            .map(|msg| {
                msg.lines()
                    .map(|line| Self::wrap(line, area.width))
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut y = area.height - 1;
        for message in unfolded_msgs.iter().rev() {
            for line in message.iter().rev() {
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
