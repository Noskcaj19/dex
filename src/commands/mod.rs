use std::sync::mpsc::Sender;

use cmd_parsing::parse_cmd;
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
        debug!("Running command: {}", cmd);
        if let Some(cmd) = parse_cmd(cmd) {
            let split_cmd: Vec<_> = cmd.command.split_whitespace().collect();
            match split_cmd.get(0).cloned().unwrap_or_default() {
                "quit" | "q" => self.event_channel.send(Event::ShutdownAll).unwrap(),
                "nick" => {
                    // Nick
                    // Todo: Add feedback when no arguments are provided
                    if let Some(new_nick) = split_cmd.get(1) {
                        debug!("Setting nickname to: {}", new_nick);
                        app.current_guild
                            .map(|guild| guild.edit_nickname(Some(new_nick)));
                    }
                }
                "clearnick" | "cnick" => {
                    app.current_guild.map(|guild| guild.edit_nickname(None));
                }
                _ => {}
            }
        }
    }
}
