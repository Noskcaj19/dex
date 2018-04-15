use std::sync::mpsc::Sender;

use models::application::Application;
use models::event::Event;

pub struct CommandHandler {
    event_channel: Sender<Event>,
}

impl CommandHandler {
    pub fn new(event_channel: Sender<Event>) -> CommandHandler {
        CommandHandler { event_channel }
    }

    pub fn execute(&self, app: &Application, cmd: &str) {
        let command = if let Some(ch) = cmd.chars().next() {
            ch
        } else {
            return;
        };

        match command {
            'q' => {
                // Quit
                self.event_channel.send(Event::ShutdownAll).unwrap()
            }
            _ => {}
        }
    }
}
