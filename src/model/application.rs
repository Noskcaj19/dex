use failure::Error;
use notify_rust::Notification;
use serenity::prelude::RwLock;
use serenity::CACHE;

use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;

use command::CommandHandler;
use discord::DiscordClient;
use helpers::signal::SignalHandler;
use model::State as SavedState;
use model::{Context, Event, MessageItem, Preferences};
use view::View;

enum State {
    NotReady,
    Ready,
    Exiting,
}

pub struct Application {
    pub view: View,
    pub discord_client: DiscordClient,
    pub context: Arc<RwLock<Context>>,
    pub command_handler: CommandHandler,
    state: State,
    events: Receiver<Event>,
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let preferences = Preferences::load()?;
        let state = SavedState::load()?;

        let state = state;

        let (event_channel, events) = mpsc::channel();

        SignalHandler::start(event_channel.clone());

        let context = Arc::new(RwLock::new(Context::new(
            &preferences,
            &state,
            event_channel,
        )));

        let view = View::new(&context.clone());

        let command_handler = CommandHandler::new();

        let discord_client = DiscordClient::start(&context.clone())?;

        Ok(Application {
            view,
            discord_client,
            context,
            command_handler,
            state: State::NotReady,
            events,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.view.message_view.load_messages(self);

        loop {
            match self.state {
                State::NotReady => {
                    println!("Loading...");
                }
                State::Ready => {
                    self.view.present()?;
                }
                State::Exiting => {
                    debug!("Exiting event loop");
                    trace!("Saving state...");
                    self.context.read().save_state()?;
                    debug!("Saved state");
                    break;
                }
            }
            self.wait_for_event();
        }
        Ok(())
    }

    pub fn wait_for_event(&mut self) {
        use termbuf::termion::event::Key;
        let event = self.events.recv();
        trace!("Event: {:?}", event);
        match event {
            Ok(Event::InternalError(err)) => {
                error!("Internal error: {}", err);
            }
            Ok(Event::DiscordReady) => {
                debug!("Discord ready");
                self.context.write().current_user = Some(CACHE.read().user.clone());
                self.state = State::Ready;

                self.view.guild_list.populate_guild_list();
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
                if Some(msg.channel_id) == self.context.read().channel {
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
            }
            Ok(Event::MessageDelete(channel_id, message_id)) => {
                self.view.message_view.delete_msg(channel_id, message_id)
            }
            Ok(Event::MessageDeleteBulk(channel_id, message_ids)) => self
                .view
                .message_view
                .delete_msg_bulk(channel_id, &message_ids),
            Ok(Event::MessageUpdateEvent(update)) => self.view.message_view.update_message(*update),
            Ok(Event::ChannelUpdateEvent) => self.view.guild_list.populate_guild_list(),
            Ok(Event::UserMessage(msg)) => {
                if self
                    .context
                    .read()
                    .channel
                    .map(|channel| channel.say(msg))
                    .is_none()
                {
                    self.send_err(format_err!("Unable to send message in current channel"))
                }
            }
            Ok(Event::SetChannel(new_chan)) => {
                self.context.write().channel = Some(new_chan);
                self.view.message_view.load_messages(self);
            }
            Ok(Event::UserCommand(cmd)) => self.command_handler.execute(self, &cmd),
            Ok(Event::UserTyping) => {
                if let Some(channel) = self.context.read().channel {
                    if let Err(err) = channel.broadcast_typing() {
                        self.send_err(format_err!("Error broadcasting typing status: {}", err));
                    }
                }
            }
            Ok(Event::TypingStart(event)) => {
                self.view.indicator.typing_start(event);
            }
            Ok(Event::WindowSizeChange) => {
                self.view.update_size();
            }
            Err(err) => error!("{:?}", err),
        }
    }

    fn send_err(&self, err: Error) {
        self.context
            .read()
            .event_channel
            .send(Event::InternalError(err))
            .unwrap()
    }
}
