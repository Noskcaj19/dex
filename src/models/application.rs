use failure::Error;
use notify_rust::Notification;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::user::CurrentUser;
use serenity::CACHE;

use std::sync::mpsc::{self, Receiver, Sender};

use super::event::Event;
use super::preferences::Preferences;
use commands::CommandHandler;
use discord::DiscordClient;
use helpers::signals::SignalHandler;
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
    pub command_handler: CommandHandler,
    state: State,
    events: Receiver<Event>,
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let preferences = Preferences::load()?;
        let (event_channel, events) = mpsc::channel();

        SignalHandler::start(event_channel.clone());

        let view = View::new(&preferences.clone(), event_channel.clone());

        let command_handler = CommandHandler::new(event_channel.clone());

        let discord_client = DiscordClient::start(&preferences.token, event_channel.clone())?;

        Ok(Application {
            view,
            discord_client,
            current_guild: preferences.guild,
            current_channel: preferences.channel,
            preferences,
            event_channel,
            command_handler,
            current_user: None,
            state: State::NotReady,
            events,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.view
            .message_view
            .load_messages(self.current_channel, self.view.terminal_size.height);

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
            Ok(Event::InternalError(err)) => {
                error!("Internal error: {}", err);
            }
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
                    if let Err(err) = self.view.input_view.key_press(key) {
                        self.send_err(format_err!("Error handling input: {}", err))
                    }
                }
            },
            Ok(Event::ShutdownAll) => {
                self.discord_client.shutdown();
                self.state = State::Exiting;
            }
            Ok(Event::NewMessage(msg)) => {
                if !msg.is_own() {
                    if let Err(e) = Notification::new()
                        .summary(&msg.author.name)
                        .body(&msg.content)
                        .show()
                    {
                        self.send_err(format_err!("Error displaying notification: {}", e));
                    }
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
                .delete_msg_bulk(channel_id, &message_ids),
            Ok(Event::MessageUpdateEvent(update)) => self.view.message_view.update_message(*update),
            Ok(Event::UserMessage(msg)) => {
                if self.current_channel
                    .map(|channel| channel.say(msg))
                    .is_none()
                {
                    self.send_err(format_err!("Unable to send message in current channel"))
                }
            }
            Ok(Event::UserCommand(cmd)) => self.command_handler.execute(self, &cmd),
            Ok(Event::UserTyping) => {
                if let Some(channel) = self.current_channel {
                    if let Err(err) = channel.broadcast_typing() {
                        self.send_err(format_err!("Error broadcasting typing status: {}", err));
                    }
                }
            }
            Ok(Event::WindowSizeChange) => {
                self.view.update_size();
            }
            Err(err) => error!("{:?}", err),
        }
    }

    fn send_err(&self, err: Error) {
        self.event_channel.send(Event::InternalError(err)).unwrap()
    }
}
