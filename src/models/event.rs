use serenity::model::channel;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::id::{ChannelId, MessageId};
use termion::event::Key;

#[derive(Debug)]
pub enum Event {
    ShutdownAll,
    NewMessage(channel::Message),
    MessageDelete(ChannelId, MessageId),
    MessageDeleteBulk(ChannelId, Vec<MessageId>),
    MessageUpdateEvent(MessageUpdateEvent),
    DiscordReady,
    Keypress(Key),
    UserCommand(String),
}
