use failure::Error;
use serenity::model::id::{ChannelId, GuildId};

use std::sync::mpsc::{self, Receiver, Sender};

use super::event::Event;
use super::preferences::Preferences;
use discord::DiscordClient;
use models::message::MessageItem;
use view::View;

pub struct Application {
    pub view: View,
    pub discord_client: DiscordClient,
    pub preferences: Preferences,
    pub current_guild: Option<GuildId>,
    pub current_channel: Option<ChannelId>,
    pub event_channel: Sender<Event>,
    events: Receiver<Event>,
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let preferences = Preferences::load()?;
        let (event_channel, events) = mpsc::channel();

        let view = View::new(preferences.clone(), event_channel.clone());

        let discord_client = DiscordClient::start(&preferences.token(), event_channel.clone())?;

        Ok(Application {
            view,
            discord_client,
            preferences,
            current_guild: None,
            current_channel: None,
            event_channel,
            events,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            self.view.present()?;

            if !self.wait_for_event() {
                debug!("Exiting event loop");
                break;
            }
        }
        Ok(())
    }

    pub fn wait_for_event(&mut self) -> bool {
        use termion::event::Key;
        let event = self.events.recv();
        trace!("Event: {:?}", event);
        match event {
            Ok(Event::Keypress(key)) => match key {
                Key::Ctrl('c') | Key::Ctrl('d') => return false,
                key => {
                    let _ = self.view.key_press(key);
                }
            },
            Ok(Event::NewMessage(msg)) => self.view.new_msg(MessageItem::DiscordMessage(msg)),
            Ok(Event::MessageDelete(channel_id, message_id)) => {
                self.view.delete_msg(channel_id, message_id)
            }
            Ok(Event::MessageDeleteBulk(channel_id, message_ids)) => {
                self.view.delete_msg_bulk(channel_id, message_ids)
            }
            _ => {}
        }
        return true;
    }
}
