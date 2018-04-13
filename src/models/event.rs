use serenity::model::channel;
use serenity::model::id::{ChannelId, MessageId};
use termion::event::Key;

#[derive(Debug)]
pub enum Event {
    ShutdownAll,
    NewMessage(channel::Message),
    MessageDelete(ChannelId, MessageId),
    DiscordReady,
    Keypress(Key),
    UserCommand(String),
}
