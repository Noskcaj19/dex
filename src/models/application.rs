use failure::Error;
use notify_rust::Notification;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::user::CurrentUser;
use serenity::CACHE;

use std::sync::mpsc::{self, Receiver, Sender};

use super::event::Event;
use super::preferences::Preferences;
use discord::DiscordClient;
use models::message::MessageItem;
use view::View;

enum State {
    NotReady,
    Ready,
    Exiting,
}

pub struct Application {
    pub view: View,
    pub discord_client: DiscordClient,
    pub preferences: Preferences,
    pub current_guild: Option<GuildId>,
    pub current_channel: Option<ChannelId>,
    pub event_channel: Sender<Event>,
    pub current_user: Option<CurrentUser>,
    state: State,
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
            current_user: None,
            state: State::NotReady,
            events,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.load_messages();

        loop {
            match self.state {
                State::NotReady => {}
                State::Ready => {
                    self.view.present()?;
                }
                State::Exiting => {
                    debug!("Exiting event loop");
                    break;
                }
            }
            self.wait_for_event();
        }
        Ok(())
    }

    pub fn wait_for_event(&mut self) {
        use termion::event::Key;
        let event = self.events.recv();
        trace!("Event: {:?}", event);
        match event {
            Ok(Event::DiscordReady) => {
                debug!("Discord ready");
                self.current_user = Some(CACHE.read().user.clone());
                self.state = State::Ready;
            }
            Ok(Event::Keypress(key)) => match key {
                Key::Ctrl('c') | Key::Ctrl('d') => {
                    self.discord_client.shutdown();
                    self.state = State::Exiting;
                }
                key => {
                    let _ = self.view.input_view.key_press(key);
                }
            },
            Ok(Event::ShutdownAll) => self.discord_client.shutdown(),
            Ok(Event::NewMessage(msg)) => {
                match &self.current_user {
                    Some(user) if user.id != msg.author.id => {
                        if let Err(e) = Notification::new()
                            .summary(&msg.author.name)
                            .body(&msg.content)
                            .show()
                        {
                            error!("Error displaying notification: {}", e);
                        }
                    }
                    _ => {}
                }

                self.view
                    .message_view
                    .add_msg(MessageItem::DiscordMessage(msg));
            }
            Ok(Event::MessageDelete(channel_id, message_id)) => {
                self.view.message_view.delete_msg(channel_id, message_id)
            }
            Ok(Event::MessageDeleteBulk(channel_id, message_ids)) => self.view
                .message_view
                .delete_msg_bulk(channel_id, message_ids),
            Ok(Event::MessageUpdateEvent(update)) => self.view.message_view.update_message(update),
            Ok(Event::UserMessage(_cmd)) => {}
            Ok(Event::UserCommand(_cmd)) => {}
            Err(err) => error!("{:?}", err),
        }
    }

    pub fn load_messages(&mut self) {
        use serenity::builder::GetMessages;

        let num_messages = self.view.terminal.size().height;
        let retriever = GetMessages::default().limit(num_messages as u64);

        if let Some(channel) = self.preferences.previous_channel() {
            for message in channel.messages(|_| retriever).unwrap().iter().rev() {
                self.view
                    .message_view
                    .add_msg(MessageItem::DiscordMessage(message.clone()));
            }
        }
    }
}
